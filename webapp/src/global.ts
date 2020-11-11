import * as queryString from 'query-string'

import {Worp} from './worp/worp'
import {WorpImpl} from './worp/worp-impl'
import {WorpMock} from './worp/worp-mock'

const params = queryString.parse(window.location.search)
const endpointUrl = typeof params.ep === 'string' ? params.ep : undefined
export const doHighlightQueryResults = params.hl === undefined ? undefined : (params.hl === null || params.hl === 'true')
export const doLogQueriesForAnalytics = params.lq === undefined ? undefined : (params.lq === null || params.lq === 'true')
export const isDevMode = params.dm === undefined ? undefined : (params.dm === null || params.dm === 'true')
export const defaultWorpId = params.wi === undefined || typeof params.wi !== 'string' ? undefined : params.wi
export const defaultQueryCollectionName = params.qc === undefined || typeof params.qc !== 'string' ? undefined : params.qc
export const usingMockWorp = endpointUrl === undefined

let worp: Worp | undefined
export function getWorp() {
  if (!worp) {
    if (!endpointUrl) {
      worp = new WorpMock()
    } else {
      worp = new WorpImpl({
        endpointUrl,
      })
    }
  }
  return worp
}
