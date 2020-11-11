import {asType} from "../util"

export type WorpDocument = {
  [key: string]: string | number | boolean | null
}

export type SearchQuery = {
  /**
   * @defaultValue '*'
   */
  query?: string
  /**
   * @defaultValue ['*']
   */
  fields?: string[]
  /**
   * @defaultValue true
   */
  doPrefixLast?: boolean
  collection?: string
}

export type SortBy = {
  /**
   * By default, sort by rank
   * @defaultValue '_score'
   */
  name?: string | '_score'
  /**
   * @defaultValue true
   */
  is_descending?: boolean
}

export type SearchRequestBody = {
  /**
   * Worp account ID
   */
  wid: string
  collection_name: string
  /**
   * @defaultValue true
   */
  do_log_query_for_analytics?: boolean
  num_results_per_page?: number
  /**
   * @defaultValue 1
   */
  page_number?: number
  worp_id?: string
  queries?: SearchQuery[]
  sort_by?: SortBy
  /**
   * @defaultValue ['*']
   */
  fields_to_return?: string[]
  /**
   * @defaultValue false
   */
  do_highlights_tagged?: boolean
  /**
   * @defaultValue false
   */
  do_highlights_map?: boolean
  /**
   * @defaultValue false
   */
  do_highlights_objects?: boolean
  /**
   * @defaultValue '<em>'
   */
  highlight_pre_tag?: string
  /**
   * @defaultValue '</em>'
   */
  highlight_post_tag?: string
  /**
   * The number of characters before and after a highlighted term. Set this to a large number in order to avoid
   * truncating the highlight context (for example: 66,000)
   *
   * @defaultValue 20
   */
  min_highlight_context?: number
  /**
   * @defaultValue 600
   */
  max_total_snippets_length?: number
}

export type HighlightsTagged = {
  [key: string]: string[]
}

export type HighlightsMapText = {
  content: string
  is_match?: boolean
}

export type HighlightsMap = {
  [key: string]: HighlightsMapText[]
}

export type SearchHit = {
  id: number
  score: number
  source: WorpDocument
  highlights_tagged?: HighlightsTagged
  highlights_map?: HighlightsMap
  /**
   * TODO: define the specific type here
   */
  highlights_objects?: any
}

/**
 * [timestampInTensOfMinutesSinceEpoch, numQueries]
 */
export type QueryHistoryTimeBinInternal = [number, number]

export type QueryHistoryResponseInternal = {
  timestamp_10min_aggregates: QueryHistoryTimeBinInternal[]
}

export type SearchResponseBody = {
  /**
   * Total number of hits
   */
  total: number
  hits: SearchHit[]
}

/**
 * This is the response from searching a hiddenQueryCollection
 */
export type HiddenSearchResponseBody = Partial<SearchResponseBody> & {
  timestamp_10min_aggregates: QueryHistoryTimeBinInternal[]
}

/**
 * Time bin duration is 10 minutes
 */
export type QueryHistoryTimeBin = {
  /**
   * In milliseconds since the epoch
   */
  startTime: number
  numQueries: number
}

export type GetQueryHistoryResult = {
  bins: QueryHistoryTimeBin[],
}

export function queryHistoryTimeBinFromInternal(bin: QueryHistoryTimeBinInternal) {
  const timestampInTensOfMinutes = bin[0]
  const numQueries = bin[1]
  const timestampInMillis = timestampInTensOfMinutes * 10 * 60 * 1000
  return asType<QueryHistoryTimeBin>({
    startTime: timestampInMillis,
    numQueries,
  })
}

export type GetQueryHistoryParams = {
  query: string
  collection: string
  worpId: string
}

export interface Worp {
  getQueryHistory(params: GetQueryHistoryParams): Promise<GetQueryHistoryResult>
  search(request: SearchRequestBody): Promise<SearchResponseBody>
}
