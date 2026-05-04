// ABOUTME: Public checker API: checkMany(), check(), Status enum, Result type.
// ABOUTME: Mirrors the Python facade in python/vacant/checker.py.

import { binding } from './native.js'
import { resolveRulesPath } from './rules-path.js'
import type { DiskCache } from './disk-cache.js'

export const Status = {
  AVAILABLE: 'available',
  REGISTERED: 'registered',
  RESERVED: 'reserved',
  INVALID: 'invalid',
  UNKNOWN: 'unknown',
} as const

export type Status = (typeof Status)[keyof typeof Status]

export interface Result {
  input: string
  domain: string
  zone: string
  status: Status
  detail: string
  fromCache: boolean
}

export interface CheckOptions {
  timeout?: number
  cache?: DiskCache | string | null
  cacheTtl?: number
}

export interface CheckManyOptions extends CheckOptions {
  concurrency?: number
}

let rulesLoaded = false

function ensureRulesLoaded(): void {
  if (rulesLoaded) return
  binding.loadRules(resolveRulesPath())
  rulesLoaded = true
}

function coerceCache(cache: DiskCache | string | null | undefined) {
  if (cache == null) return null
  if (typeof cache === 'string') {
    return new binding.DiskCache(cache)
  }
  const inner = (cache as unknown as { _inner?: unknown })._inner
  if (inner) return inner as InstanceType<typeof binding.DiskCache>
  return new binding.DiskCache(String(cache))
}

function toResult(row: {
  input: string
  domain: string
  zone: string
  status: string
  detail: string
  fromCache: boolean
}): Result {
  return {
    input: row.input,
    domain: row.domain,
    zone: row.zone,
    status: row.status as Status,
    detail: row.detail,
    fromCache: row.fromCache,
  }
}

export function checkMany(
  domains: string[],
  options: CheckManyOptions = {},
): Result[] {
  ensureRulesLoaded()
  const { timeout = 4.0, concurrency = 64, cache, cacheTtl = 86_400.0 } = options
  const rustCache = coerceCache(cache)
  const rows = binding.checkMany(domains, concurrency, timeout, rustCache as never, cacheTtl)
  return rows.map(toResult)
}

export function check(domain: string, options: CheckOptions = {}): Result {
  return checkMany([domain], { ...options, concurrency: 1 })[0]!
}
