// ABOUTME: Single import point for the napi-rs generated bindings.
// ABOUTME: Loads the package-root index.js loader at runtime via createRequire.

import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

declare const __dirname: string | undefined

export interface CheckRowNative {
  input: string
  domain: string
  zone: string
  status: string
  detail: string
  fromCache: boolean
}

export interface CachedRowNative {
  domain: string
  zone: string
  status: string
  detail: string
  checkedAt: number
}

export interface NativeDiskCache {
  get(domain: string, ttl: number): CachedRowNative | null
  put(domain: string, zone: string, status: string, detail: string): void
}

export interface NativeBinding {
  loadRules(path: string): void
  checkMany(
    domains: string[],
    concurrency?: number,
    timeout?: number,
    cache?: NativeDiskCache | null,
    cacheTtl?: number,
    verify?: boolean,
  ): CheckRowNative[]
  DiskCache: {
    new (path?: string | null): NativeDiskCache
    defaultPath(): string
  }
}

function dirHere(): string {
  if (typeof __dirname !== 'undefined') return __dirname
  return dirname(fileURLToPath(import.meta.url))
}

function loadNative(): NativeBinding {
  // dist/index.{cjs,js} or dist/bin/vacant.js -> walk up to the package root,
  // then load the napi-generated loader. Walk up looking for the file that
  // exports loadRules (i.e. the napi loader, not a sibling tsup bundle).
  const here = dirHere()
  const candidates = [
    join(here, '..', 'index.cjs'),
    join(here, '..', '..', 'index.cjs'),
    join(here, '..', '..', '..', 'index.cjs'),
  ]
  const require_ = createRequire(join(here, 'noop.js'))
  let lastError: unknown
  for (const path of candidates) {
    try {
      const mod = require_(path) as Partial<NativeBinding>
      if (typeof mod.loadRules === 'function') {
        return mod as NativeBinding
      }
    } catch (err) {
      lastError = err
    }
  }
  throw new Error(
    `failed to load @alltuner/vacant native binding from ${candidates.join(
      ' or ',
    )}: ${lastError instanceof Error ? lastError.message : String(lastError)}`,
  )
}

export const binding: NativeBinding = loadNative()
