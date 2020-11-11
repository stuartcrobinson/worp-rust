export type OptionalPropertyOf<T extends object> = Exclude<{
  [K in keyof T]: T extends Record<K, T[K]> ? never : K
}[keyof T], undefined>

export type RequiredPropertyOf<T extends object> = Exclude<{
  [K in keyof T]: T extends Record<K, T[K]> ? K : never
}[keyof T], undefined>

export type PropertyTypeExtends<T extends object, Ancestor> = Exclude<{
  [K in keyof T]: T[K] extends Ancestor ? K : never
}[keyof T], undefined>

export type PropertyTypeSuperOf<T extends object, Descendant> = Exclude<{
  [K in keyof T]: Descendant extends T[K] ? K : never
}[keyof T], undefined>

export function asType<T>(value: T) {
  return value
}

export function* matches(text: string, pattern: RegExp) {
  const clone = new RegExp(pattern.source, pattern.flags)
  let match: RegExpExecArray | null = null
  do {
    match = clone.exec(text)
    if (match) {
      yield match
    }
  } while (match)
}

/**
 * Return the supplied string if a condition is satisfied, otherwise
 * return an empty string or the orElse parameter.
 *
 * @param str the string to return if ifCondition is true
 * @param ifCondition
 * @param orElse the string to return if ifCondition is not true. The default is an empty string
 */
export function strIf(
    str: (() => string) | string,
    ifCondition: boolean,
    orElse?: (() => string) | string) {

  const positive = typeof str === 'string' ? () => str : str
  const negative = (orElse === undefined || typeof orElse === 'string') ? () => (orElse || '') : orElse

  return ifCondition ? positive() : negative()
}

export function pluralize(
    plural: (() => string) | string,
    count: number | undefined,
    singular?: (() => string) | string) {

  count = count || 0

  return strIf(plural, count !== 1, singular)
}

export function toFirstLetterUppercase(text: string | undefined) {
  if (!text) {
    return text
  }
  return text[0].toUpperCase() + text.slice(1)
}

export function deepIdentical(a: any, b: any) {
  // TODO: arrange object keys in standard ordering
  return JSON.stringify(a) === JSON.stringify(b)
}

/**
 * Return seconds since Jan 1, 1970 (UTC)
 */
export function dateToSecondsSinceEpoch(date: Date) {
  return Math.floor(date.getTime() / 1000)
}
