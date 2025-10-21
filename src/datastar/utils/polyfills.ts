export const hasOwn: (obj: object, prop: PropertyKey) => boolean =
  Object.hasOwn ?? Object.prototype.hasOwnProperty.call
