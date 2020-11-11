import {
  GetQueryHistoryParams,
  GetQueryHistoryResult,
  HighlightsMap,
  HighlightsMapText,
  QueryHistoryTimeBinInternal,
  SearchHit,
  SearchRequestBody,
  SearchResponseBody,
  Worp,
  WorpDocument,
  queryHistoryTimeBinFromInternal,
} from './worp'

import {asType} from '../util'

const foods: WorpDocument[] = [
  {
    name: 'beans',
    description: 'beans are kind of round and small, about the size of a finger nail or smaller.',
  },
  {
    name: 'encheladas',
    description: 'encheladas are delicious and best served hot',
  },
  {
    name: 'pizza',
    description: 'italian pizza is my favorite. the crust is not very thick and the ingredients are minimal, but very fresh',
  },
]

/**
 * The result is rounded to a UTC offset
 */
function getRoundedDate(minutes: number, date?: Date) {
  if (!date) {
    date = new Date()
    let testDst = false
    if (testDst) {
      let testBegin = true
      if (testBegin) {
        // test daylight savings begin
        date = new Date('2020-03-08T12:00:00Z')
      } else {
        // test daylight savings end
        date = new Date('2020-11-01T12:00:00Z')
      }
    }
  }

  // convert minutes to ms
  let ms = 1000 * 60 * minutes
  let roundedDate = new Date(Math.round(date.getTime() / ms) * ms)
  return roundedDate
}

export class WorpMock implements Worp {
  async getQueryHistory(_params: GetQueryHistoryParams): Promise<GetQueryHistoryResult> {
    await new Promise(resolve => setTimeout(resolve, 0))
    const binsInternal: QueryHistoryTimeBinInternal[] = []
    let currentTimeTensOfMinutes = Math.floor(getRoundedDate(10).getTime() / 1000 / 60 / 10)
    // provide 30 days of history
    const howFarBack = currentTimeTensOfMinutes - (30 * 24 * 60 / 10)
    const maxCount = 20
    const minCount = 0
    const countRange = maxCount - minCount
    const maxChangePerStep = countRange
    let currentCount = minCount + Math.round(Math.random() * countRange)
    let internalCount = 0
    for (; currentTimeTensOfMinutes > howFarBack; --currentTimeTensOfMinutes) {
      if (internalCount++ > 50) {
        internalCount = 0
        await new Promise(resolve => setTimeout(resolve, 0))
      }
      const countChange = Math.round((Math.random() - 0.5) * 2 * maxChangePerStep)
      currentCount = Math.min(maxCount, Math.max(minCount, currentCount + countChange))
      binsInternal.push([currentTimeTensOfMinutes, currentCount])
    }
    let detailDebug = false
    detailDebug && console.debug('query history bins:', binsInternal)
    return asType<GetQueryHistoryResult>({
      bins: binsInternal.map(bin => queryHistoryTimeBinFromInternal(bin)),
    })
  }
  async search(request: SearchRequestBody): Promise<SearchResponseBody> {
    await new Promise(resolve => setTimeout(resolve, 0))
    let queryString = ''
    if (request.queries) {
      const query = request.queries[0]
      if (query.query) {
        queryString = query.query
      }
    }
    queryString = queryString.trim()
    const hits: SearchHit[] = []
    foods.forEach((source, index) => {
      let highlightsMap: HighlightsMap = {}
      let isHit = false
      if (!queryString) {
        isHit = true
      } else {
        for (const field in source) {
          const value = source[field]
          if (typeof value === 'string') {
            if (value.includes(queryString)) {
              isHit = true
              const texts: HighlightsMapText[] = []
              if (value === queryString) {
                texts.push({
                  content: queryString,
                  is_match: true,
                })
              } else {
                const nonMatches = value.split(queryString)
                nonMatches.forEach((nonMatch, index) => {
                  texts.push({
                    content: nonMatch,
                  })
                  if (index + 1 < nonMatches.length || nonMatch) {
                    texts.push({
                      content: queryString,
                      is_match: true,
                    })
                  }
                })
                if (!value.endsWith(queryString)) {
                  texts.pop()
                }
              }
              highlightsMap[field] = texts
            }
          }
        }
      }
      if (isHit) {
        hits.push({
          id: index,
          score: 100,
          source,
          highlights_map: highlightsMap,
        })
      }
    })
    await new Promise(resolve => setTimeout(resolve, 10))
    return {
      total: hits.length,
      hits,
    }
  }
}
