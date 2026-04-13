export function serialize<T extends Serializable>(value: T): string {
  return JSON.stringify(value)
}
