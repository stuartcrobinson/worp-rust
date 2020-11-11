import {
  GetQueryHistoryParams,
  GetQueryHistoryResult,
  HiddenSearchResponseBody,
  SearchRequestBody,
  SearchResponseBody,
  Worp,
  queryHistoryTimeBinFromInternal,
} from './worp'

import Axios from 'axios'
import {asType} from '../util'

export class WorpImpl implements Worp {
  private endpointUrl: string
  constructor(config: {
    endpointUrl: string
  }) {
    this.endpointUrl = config.endpointUrl
  }

  async getQueryHistory(params: GetQueryHistoryParams): Promise<GetQueryHistoryResult> {
    const response = await this.search({
      wid: params.worpId,
      collection_name: `${params.collection}_hiddenQueryCollection`,
      queries: [
        {
          query: params.query,
          fields: ['*'],
        },
      ]
    }) as HiddenSearchResponseBody
    return asType<GetQueryHistoryResult>({
      bins: response.timestamp_10min_aggregates.map(bin => queryHistoryTimeBinFromInternal(bin)),
    })
  }

  async search(request: SearchRequestBody): Promise<SearchResponseBody> {
    return (
      await Axios.post(`${this.endpointUrl}/query`, request)
    ).data as SearchResponseBody
  }
}
