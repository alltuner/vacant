// ABOUTME: DiskCache wrapper around the napi-rs binding's DiskCache.
// ABOUTME: Same on-disk SQLite shape as the vacant CLI; safe to share.

import { binding } from './native.js'
import { Status, type Result } from './checker.js'

export interface CachedEntry {
  domain: string
  zone: string
  status: Status
  detail: string
  checkedAt: number
}

export function defaultPath(): string {
  return binding.DiskCache.defaultPath()
}

export interface DiskCacheOptions {
  path?: string
}

export class DiskCache {
  readonly path: string
  // Native handle; named with a leading underscore to mark it as internal.
  // checker.ts unwraps it via duck-typing.
  private readonly _inner: InstanceType<typeof binding.DiskCache>

  constructor(options: DiskCacheOptions | string = {}) {
    const path =
      typeof options === 'string'
        ? options
        : options.path ?? binding.DiskCache.defaultPath()
    this.path = path
    this._inner = new binding.DiskCache(path)
  }

  get(domain: string, ttl: number): CachedEntry | null {
    const row = this._inner.get(domain, ttl)
    if (!row) return null
    return {
      domain: row.domain,
      zone: row.zone,
      status: row.status as Status,
      detail: row.detail,
      checkedAt: row.checkedAt,
    }
  }

  put(result: Result): void {
    if (
      result.status === Status.UNKNOWN ||
      result.status === Status.INVALID ||
      !result.domain
    )
      return
    this._inner.put(result.domain, result.zone, result.status, result.detail)
  }
}
