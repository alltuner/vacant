// ABOUTME: Public entry point — re-exports the JS surface for `@alltuner/vacant`.
// ABOUTME: Mirrors python/vacant/__init__.py.

export { check, checkMany, Status } from './checker.js'
export type { CheckManyOptions, CheckOptions, Result } from './checker.js'
export { DiskCache, defaultPath } from './disk-cache.js'
export type { CachedEntry, DiskCacheOptions } from './disk-cache.js'
