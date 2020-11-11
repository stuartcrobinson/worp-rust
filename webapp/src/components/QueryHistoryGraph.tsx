import * as React from 'react'

import {Chart, ChartConfiguration, ChartPoint} from 'chart.js'

import {Bar} from 'react-chartjs-2'
import {QueryHistoryTimeBin} from '../worp'
import {asType} from '../util'
import formatTime from 'date-fns/format'
import {isDevMode} from '../global'

/**
 * These values are in minutes
 */
export enum BinSize {
  TenMin = 10,
  Hour = 60,
  Day = 24 * 60,
}

type GraphType = 'line' | 'bar'

type Props = {
  data: QueryHistoryTimeBin[]
  /**
   * @defaultValue 'day'
   */
  binSize?: BinSize
  query?: string
}

export function QueryHistoryGraph(props: Props) {
  const {binSize: binSizeMaybe, data, query} = props
  const binSize = binSizeMaybe === undefined ? BinSize.Day : binSizeMaybe

  let debug = !!isDevMode
  if (debug) {
    console.debug('history graph bins:', data)
  }

  let shouldDisplayTimesInUtc = false

  let tooltipDateFormat = 'MMM d, y'
  if (binSize < BinSize.Hour) {
    tooltipDateFormat += ` h:mm aaaaa'm'`
  } else if (binSize < BinSize.Day || !shouldDisplayTimesInUtc) {
    tooltipDateFormat += ` ha`
  }
  tooltipDateFormat += ' z'

  const graphType = asType<GraphType>('bar')

  const color = Chart.helpers.color
  const barColor = 'red'
  const chartConfig = asType<ChartConfiguration>({
    data: {
      datasets: [{
        label: query || '<query>',
        backgroundColor: color(barColor).alpha(0.5).rgbString(),
        borderColor: color(barColor).alpha(0).rgbString(),
        categoryPercentage: 1,
        barPercentage: .85,
        data: data.map(bin => ({x: bin.startTime, y: bin.numQueries})),
        type: graphType,
        pointRadius: 0,
        fill: false,
        lineTension: 0,
        borderWidth: 2,
      }],
    },
    options: {
      animation: {
        duration: 500,
      },
      scales: {
        xAxes: [{
          type: 'time',
          offset: true,
          ticks: {
            major: {
              enabled: true,
            },
            // Left over code from finance chart.js demo. Needs to be modified to work with current version of
            // chart.js
            /*
            font: context => {
              return context.tick && context.tick.major ? {style: 'bold'} : undefined;
            },
            */
            source: 'data',
            autoSkip: true,
            autoSkipPadding: 75,
            maxRotation: 0,
            sampleSize: 100
          },
          // Left over code from finance chart.js demo. Needs to be modified to work with current version of
          // chart.js
          /*
          // Custom logic that chooses major ticks by first timestamp in time period
          // E.g. if March 1 & 2 are missing from dataset because they're weekends, we pick March 3 to be beginning of month
          afterBuildTicks: (scale, ticks) => {
            const majorUnit = scale._majorUnit;
            const firstTick = ticks[0];

            let val = new Date(ticks[0].value)
            if ((majorUnit === 'minute' && val.getSeconds() === 0)
              || (majorUnit === 'hour' && val.getMinutes() === 0)
              || (majorUnit === 'day' && val.getHours() === 9)
              || (majorUnit === 'month' && val.day <= 3 && val.weekday === 1)
              || (majorUnit === 'year' && val.getMonth() === 1)) {
              firstTick.major = true
            } else {
              firstTick.major = false
            }
            let lastMajor = val.get(majorUnit);

            const newTicks
            for (let i = 1; i < ticks.length; i++) {
              const tick = ticks[i];
              val = luxon.DateTime.fromMillis(tick.value);
              const currMajor = val.get(majorUnit);
              tick.major = currMajor !== lastMajor;
              lastMajor = currMajor;
            }
            scale.ticks = ticks
          }
          */
        }],
        yAxes: [{
          type: 'linear',
          gridLines: {
            drawBorder: false
          },
          scaleLabel: {
            display: true,
            labelString: 'Query Count',
          },
          ticks: {
            beginAtZero: true,
            callback: valueParam => {
              const value = valueParam as number
              if (Number.isInteger(value)) {
                return value
              }
            },
          },
        }],
      },
      tooltips: {
        intersect: false,
        mode: 'index',
        callbacks: {
          title: (itemArray, data) => {
            return itemArray.map(item => {
              let label = item.label || ''
              if (item.datasetIndex !== undefined) {
                const dataPoint = (data.datasets![item.datasetIndex].data![item.index!] as ChartPoint).x as number
                label = formatTime(dataPoint, tooltipDateFormat)
              }
              return label
            })
          },
          label: (item, data) => {
            let label = item.label || ''
            if (item.datasetIndex !== undefined) {
              label = data.datasets![item.datasetIndex].label || ''
              if (item.yLabel !== undefined) {
                if (label) {
                  label += ': '
                }
                label += String(item.yLabel)
              }
            }
            return label
          }
        }
      },
      maintainAspectRatio: false,
    },
  })
  if (!chartConfig.data) throw new Error('missing data')

  return (
    <Bar
      data={chartConfig.data}
      options={chartConfig.options}
    />
  )
}
