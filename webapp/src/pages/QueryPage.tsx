import * as React from 'react'

import {BinSize, QueryHistoryGraph} from '../components/QueryHistoryGraph'
import {GetQueryHistoryResult, QueryHistoryTimeBin, SearchResponseBody} from '../worp/worp'
import {
  defaultQueryCollectionName,
  defaultWorpId,
  doHighlightQueryResults,
  doLogQueriesForAnalytics,
  getWorp,
  isDevMode,
  usingMockWorp,
} from '../global'

import Card from '@material-ui/core/Card'
import CardContent from '@material-ui/core/CardContent'
import FormControlLabel from '@material-ui/core/FormControlLabel'
import List from '@material-ui/core/List'
import ListItem from '@material-ui/core/ListItem'
import ListItemText from '@material-ui/core/ListItemText'
import {RateLimiter} from '../rate-limiter'
import Switch from '@material-ui/core/Switch'
import TextField from '@material-ui/core/TextField'
import ToggleButton from '@material-ui/lab/ToggleButton'
import ToggleButtonGroup from '@material-ui/lab/ToggleButtonGroup'
import Typography from '@material-ui/core/Typography'
import addMinutes from 'date-fns/addMinutes'
import {asType} from '../util'
import differenceInMinutes from 'date-fns/differenceInMinutes'
import makeStyles from '@material-ui/styles/makeStyles'
import startOfDayLocal from 'date-fns/startOfDay'
import startOfHourLocal from 'date-fns/startOfHour'

type QueryHistoryGraphData = {
  bins: QueryHistoryTimeBin[]
  query?: string
}

enum BinTimeZone {
  Utc = 'utc',
  Local = 'local',
}

export function QueryPage() {
  const classes = useStyles()
  const [worpId, setWorpId] = React.useState(
    defaultWorpId === undefined ? '' : defaultWorpId)
  const [collectionName, setCollectionName] = React.useState(
    defaultQueryCollectionName === undefined ? '' : defaultQueryCollectionName)
  const [query, setQuery] = React.useState('')
  const [isRunning, setIsRunning] = React.useState(false)
  const [doHighlights, setDoHighlights] = React.useState(
    doHighlightQueryResults === undefined ? true : doHighlightQueryResults)
  const [doLogQueries, setDoLogQueries] = React.useState(
    doLogQueriesForAnalytics === undefined ? false : doLogQueriesForAnalytics)
  const [searchResult, setSearchResult] = React.useState(asType<SearchResponseBody | undefined>(undefined))
  const [historyResult, setHistoryResult] = React.useState(asType<GetQueryHistoryResult | undefined>(undefined))
  const [queryHistoryGraphData, setQueryHistoryGraphData] = React.useState(asType<QueryHistoryGraphData>({bins: []}))
  const [historyBinSize, setHistoryBinSize] = React.useState(BinSize.Day)
  const [historyBinTimeZone, setHistoryBinTimeZone] = React.useState(BinTimeZone.Local)
  const [elapsedMillis, setElapsedMillis] = React.useState(-1)

  React.useEffect(() => {
    return () => cleanup()
  }, [])

  const onWorpIdChange = (event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setWorpId(event.target.value)
  }

  const onCollectionNameChange = (event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setCollectionName(event.target.value)
  }

  const onQueryChange = (event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const newQuery = event.target.value
    setQuery(newQuery)
    if (newQuery && ((worpId && collectionName) || usingMockWorp)) {
      runQuery(newQuery, doHighlights)
    } else {
      searchRequester.cancelRequests()
      historyRequester.cancelRequests()
      setIsRunning(false)
      setElapsedMillis(-1)
      setSearchResult(undefined)
      setHistoryResult(undefined)
      setQueryHistoryGraphData({bins: []})
    }
  }

  const binQueryHistory = (originalBins: QueryHistoryTimeBin[], binSize: BinSize, binTimeZone: BinTimeZone,
      query?: string) => {
    if (originalBins.length) {
      const bins = [...originalBins].sort((a, b) => a.startTime - b.startTime)
      let newBins: QueryHistoryTimeBin[] = []
      let currentCount = 0
      let currentBinStartTime = 0
      let nextBinStartTime = 0
      const startOfDay: (date: Date | number) => Date = binTimeZone === BinTimeZone.Utc ?
        date => {
          const newDate = new Date(date)
          newDate.setUTCHours(0, 0, 0, 0)
          return newDate
        } :
        date => startOfDayLocal(date)
      const startOfHour: (date: Date | number) => Date = binTimeZone === BinTimeZone.Utc ?
        date => {
          const newDate = new Date(date)
          newDate.setUTCMinutes(0, 0, 0)
          return newDate
        } :
        date => {
          let hourStart = startOfHourLocal(date)
          // check for daylight savings weirdness
          const minuteDiff = differenceInMinutes(date, hourStart)
          if (minuteDiff >= 60) {
            hourStart = addMinutes(hourStart, 60)
          }
          return hourStart
        }
      const startOfSixthOfHour: (date: Date | number) => Date = date => {
        const hourStart = startOfHour(date)
        const minuteDiff = Math.floor(differenceInMinutes(date, hourStart) / 10) * 10
        const result = addMinutes(hourStart, minuteDiff)
        let detailDebug = false
        if (detailDebug) {
          console.debug('startOfSixthOfHour(', new Date(date), ') minuteDiff:', minuteDiff, ', result:', result)
        }
        return result
      }
      let getBinStartTime: (time: Date | number) => Date
      let slicer: (bins: QueryHistoryTimeBin[]) => QueryHistoryTimeBin[]
      let paddedBinCount = 0
      switch (binSize) {
        case BinSize.Day:
          getBinStartTime = startOfDay
          slicer = bins => bins
          break
        case BinSize.Hour:
          getBinStartTime = startOfHour
          slicer = bins => {
            // 1 week of hourly bins
            const maxBins = 168
            const startIndex = Math.max(0, bins.length - maxBins - paddedBinCount)
            return bins.slice(startIndex)
          }
          break
        case BinSize.TenMin:
          getBinStartTime = startOfSixthOfHour
          slicer = bins => {
            // 1 day of 10-minutely bins
            const maxBins = 144
            const startIndex = Math.max(0, bins.length - maxBins - paddedBinCount)
            return bins.slice(startIndex)
          }
          break
        default:
          throw new Error(`unsupported binSize: ${binSize}`)
      }
      /**
       * Tries to account for daylight savings bin offset changes
       */
      const getNextBinStartTime = (binStartDate: Date) => {
        let untilDate = addMinutes(binStartDate, binSize)
        let result = getBinStartTime(untilDate)
        // if untilDate is too early, nudge it forward until it's in the next bin
        if (result.getTime() <= binStartDate.getTime()) {
          while (result.getTime() <= binStartDate.getTime()) {
            untilDate = addMinutes(untilDate, 5)
            result = getBinStartTime(untilDate)
          }
        }
        return untilDate
      }
      const getPreviousBinStartTime = (binStartDate: Date) => {
        let prevStartDate = getBinStartTime(binStartDate.getTime() - 1)
        let result = prevStartDate
        while (prevStartDate.getTime() >= binStartDate.getTime()) {
          prevStartDate = addMinutes(prevStartDate, -5)
          result = getBinStartTime(prevStartDate)
        }
        return result
      }
      const getBin = (time: number) => {
        const startDate = getBinStartTime(time)
        const untilDate = getNextBinStartTime(startDate)
        return {
          startTime: startDate.getTime(),
          untilTime: untilDate.getTime(),
        }
      }
      bins.forEach(bin => {
        if (bin.startTime >= nextBinStartTime) {
          if (currentBinStartTime) {
            newBins.push({
              startTime: currentBinStartTime,
              numQueries: currentCount,
            })
          }
          let lastNextStartTime = nextBinStartTime
          {
            ({startTime: currentBinStartTime, untilTime: nextBinStartTime} = getBin(bin.startTime))
            currentCount = bin.numQueries
          }
          if (lastNextStartTime) {
            while (lastNextStartTime < currentBinStartTime) {
              newBins.push({
                startTime: lastNextStartTime,
                numQueries: 0,
              })
              lastNextStartTime = getNextBinStartTime(new Date(lastNextStartTime)).getTime()
            }
          }
        } else {
          currentCount += bin.numQueries
        }
      })
      // add the last bin
      newBins.push({
        startTime: currentBinStartTime,
        numQueries: currentCount,
      })

      let testSmallRange = false
      if (testSmallRange) {
        newBins.forEach(bin => {
          bin.numQueries = Math.round(Math.random() * 2)
        })
      }
      let usingRecharts = false
      let usingChartJs = true
      let padBins = usingRecharts
      let padLeft = padBins || (usingRecharts && newBins.length === 1)
      if (padLeft) {
        // add an empty bin before the first bin to work around a bug in the chart when there is only 1
        // data point (https://github.com/recharts/recharts/issues/2127)
        const firstBin = newBins[0]
        newBins.unshift({
          startTime: getPreviousBinStartTime(new Date(firstBin.startTime)).getTime(),
          numQueries: 0,
        })
        ++paddedBinCount
      }
      let padRight = padBins
      if (padRight) {
        // add an empty bin after the last bin so that the last bar in the chart doesn't extend beyond
        // the end of the x axis.
        const lastBin = newBins[newBins.length - 1]
        newBins.push({
          startTime: getNextBinStartTime(new Date(lastBin.startTime)).getTime(),
          numQueries: 0,
        })
        ++paddedBinCount
      }
      newBins = slicer(newBins)
      let detailDebug = false
      detailDebug && console.debug('new bins:', newBins)
      detailDebug && newBins.forEach(
        bin => console.debug('bin date:', new Date(bin.startTime), 'query count:', bin.numQueries)
      )
      setQueryHistoryGraphData({bins: newBins, query})
    } else {
      setQueryHistoryGraphData({bins: [], query})
    }
  }

  const fetchQueryHistory = (query: string) => historyRequester.limiter.run({
    handler: async immediate => {
      if (immediate) {
        const request = historyRequester.initRequest()
        let searchQuery = query
        if (!searchQuery.trim()) {
          searchQuery = ''
        }
        const historyResult = await getWorp().getQueryHistory({
          worpId,
          collection: collectionName,
          query: searchQuery,
        })
        let detailDebug = false
        detailDebug && console.debug('query history result:', historyResult)
        detailDebug && historyResult.bins.forEach(
          bin => console.debug('bin date:', new Date(bin.startTime), 'query count:', bin.numQueries)
        )
        if (request.wasCanceled) {
          return
        }
        setHistoryResult(historyResult)
        binQueryHistory(historyResult.bins, historyBinSize, historyBinTimeZone, query)
        historyRequester.clearRequest()
      }
    }
  })

  const runQuery = (query: string, doHighlights: boolean) => searchRequester.limiter.run({
    handler: async immediate => {
      if (immediate) {
        const request = searchRequester.initRequest()
        setIsRunning(true)
        setElapsedMillis(-1)
        let searchQuery = query
        if (!searchQuery.trim()) {
          searchQuery = ''
        }
        const startTime = Date.now()
        const searchResult = await getWorp().search({
          wid: worpId,
          collection_name: collectionName,
          do_highlights_map: doHighlights,
          do_log_query_for_analytics: doLogQueries,
          queries: [
            {
              query: searchQuery,
            },
          ],
        })
        const endTime = Date.now()
        if (request.wasCanceled) {
          return
        }
        setIsRunning(false)
        setElapsedMillis(endTime - startTime)
        setSearchResult(searchResult)
        await fetchQueryHistory(query)
        searchRequester.clearRequest()
      }
    },
  })

  const handleDoHighlightsChange = (_event: React.ChangeEvent<HTMLInputElement>) => {
    const newDoHighlights = !doHighlights
    setDoHighlights(newDoHighlights)
    if (query && ((worpId && collectionName) || usingMockWorp)) {
      runQuery(query, doHighlights)
    }
  }

  const handleDoLogQueriesChange = (_event: React.ChangeEvent<HTMLInputElement>) => {
    const newDoLogQueries = !doLogQueries
    setDoLogQueries(newDoLogQueries)
  }

  const statusParts: string[] = []
  if (elapsedMillis >= 0) {
    statusParts.push(`Elapsed time: ${elapsedMillis / 1000}s`)
  }
  if (searchResult) {
    statusParts.push(`Total hits: ${searchResult.total}`)
  }
  if (isRunning) {
    statusParts.push('Searching...')
  }

  const handleBinSizeChange = (_event: any, binSize: BinSize | null) => {
    if (binSize === null) {
      return
    }
    setHistoryBinSize(binSize)
    if (historyResult) {
      binQueryHistory(historyResult.bins, binSize, historyBinTimeZone, query)
    }
  }

  const handleBinTimeZoneChange = (_event: any, binTimeZone: BinTimeZone | null) => {
    if (binTimeZone === null) {
      return
    }
    setHistoryBinTimeZone(binTimeZone)
    if (historyResult) {
      binQueryHistory(historyResult.bins, historyBinSize, binTimeZone, query)
    }
  }

  return (
    <div className={classes.root}>
      <div style={{width: '100%', height: '200px'}}>
        <QueryHistoryGraph
          data={queryHistoryGraphData.bins}
          binSize={historyBinSize}
          query={queryHistoryGraphData.query}
        />
      </div>
      <div style={{width: '100%', textAlign: 'left'}}>
        <ToggleButtonGroup
          exclusive
          size="small"
          value={historyBinSize}
          onChange={handleBinSizeChange}
        >
          <ToggleButton value={BinSize.Day} aria-label="all">all</ToggleButton>
          <ToggleButton value={BinSize.Hour} aria-label="week">week</ToggleButton>
          <ToggleButton value={BinSize.TenMin} aria-label="day">day</ToggleButton>
        </ToggleButtonGroup>
        <FormControlLabel
          labelPlacement="start"
          control={
            <ToggleButtonGroup
              exclusive
              size="small"
              value={historyBinTimeZone}
              onChange={handleBinTimeZoneChange}
              style={{paddingLeft: '10px'}}
            >
              <ToggleButton value={BinTimeZone.Local} aria-label="local">local</ToggleButton>
              <ToggleButton value={BinTimeZone.Utc} aria-label="utc">utc</ToggleButton>
            </ToggleButtonGroup>
          }
          label="Bin Offset"
          style={{paddingLeft: '50px'}}
        />
      </div>
      <Typography variant="h4" gutterBottom>
        Run a query
      </Typography>
      {isDevMode &&
        <TextField
          label="worpId"
          placeholder="worpId"
          error={!usingMockWorp && !worpId && !!query}
          helperText={!usingMockWorp && !worpId && !!query ? 'required' : ''}
          onChange={onWorpIdChange}
          value={worpId}
        />
      }
      {isDevMode &&
        <TextField
          label="Collection"
          placeholder="collection name"
          error={!usingMockWorp && !collectionName && !!query}
          helperText={!usingMockWorp && !collectionName && !!query ? 'required' : ''}
          onChange={onCollectionNameChange}
          value={collectionName}
          style={{paddingLeft: '10px'}}
        />
      }
      {isDevMode &&
        <FormControlLabel
          control={
            <Switch
              checked={doHighlights}
              onChange={handleDoHighlightsChange}
              name="doHighlights"
              color="primary"
            />
          }
          label="Highlights"
        />
      }
      {isDevMode &&
        <FormControlLabel
          control={
            <Switch
              checked={doLogQueries}
              onChange={handleDoLogQueriesChange}
              name="doLogQueriesForAnalytics"
              color="primary"
            />
          }
          label="Log Queries For Analytics"
        />
      }
      <div>
        <TextField
          label="Query"
          placeholder="search query"
          fullWidth
          onChange={onQueryChange}
          value={query}
        />
        {statusParts.length > 0 && <p>{statusParts.join(' | ')}</p>}
      </div>
      <div style={{
        height: queryHistoryGraphData ? '32%' : '60%',
        overflow: 'auto',
      }}>
        {searchResult && searchResult.hits.map(result => {
          return (
            <Card variant="outlined" key={result.id}>
              <CardContent>
                <List>
                  <ListItem key="_id">
                    <ListItemText primary="ID" secondary={result.id} />
                  </ListItem>
                  <ListItem key="_score">
                    <ListItemText primary="Score" secondary={result.score} />
                  </ListItem>
                  {Object.keys(result.source).map(field => {
                    let value: JSX.Element[] = [<span>{result.source[field]}</span>]
                    if (doHighlights && result.highlights_map && result.highlights_map[field]) {
                      value = result.highlights_map[field].map(text => {
                        if (text.is_match) {
                          return <span style={{backgroundColor: 'yellow'}}>{text.content}</span>
                        }
                        return <span>{text.content}</span>
                      })
                    }
                    return (
                      <ListItem key={field}>
                        <ListItemText primary={field} secondary={value} />
                      </ListItem>
                    )
                  })}
                </List>
              </CardContent>
            </Card>
          )
        })}
      </div>
    </div>
  )
}

class RateLimitedRequester {
  private _limiter: RateLimiter | undefined
  private requests: {
    wasCanceled?: boolean
  }[] = []

  constructor(private config: {
    delayInMillis: number
  }) {
  }

  get limiter() {
    if (!this._limiter) {
      this._limiter = new RateLimiter({
        delayInMillis: this.config.delayInMillis,
      })
    }
    return this._limiter
  }

  get currentRequest() {
    return this.requests[this.requests.length - 1]
  }

  initRequest() {
    this.requests.push({})
    return this.currentRequest
  }

  clearRequest() {
    this.requests.pop()
  }

  cancelRequests() {
    if (this._limiter) {
      this._limiter.clear()
    }
    this.requests.forEach(request => {
      request.wasCanceled = true
    })
    this.requests = []
  }

  destroyLimiter() {
    if (this._limiter) {
      this._limiter.destroy()
      this._limiter = undefined
    }
  }

  destroy() {
    this.cancelRequests()
    this.destroyLimiter()
  }
}

const searchRequester = new RateLimitedRequester({delayInMillis: 200})
const historyRequester = new RateLimitedRequester({delayInMillis: 1000})

function cleanup() {
  searchRequester.destroy()
  historyRequester.destroy()
}

const useStyles = makeStyles({
  root: {
    height: '100%',
    textAlign: 'center',
    paddingTop: 20,
    paddingLeft: 15,
    paddingRight: 15,
  },

  button: {
    marginTop: 20,
  },
})
