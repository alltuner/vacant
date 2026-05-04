// ABOUTME: Resolves the bundled rules.toml path in both ESM and CJS builds.
// ABOUTME: Walks up from the bundle's directory until it finds vacant/rules.toml.

import { existsSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

declare const __dirname: string | undefined

function dirHere(): string {
  if (typeof __dirname !== 'undefined') return __dirname
  return dirname(fileURLToPath(import.meta.url))
}

export function resolveRulesPath(): string {
  // dist/index.{cjs,js} -> ../vacant/rules.toml
  // dist/bin/vacant.js  -> ../../vacant/rules.toml
  // unbundled src/      -> ../vacant/rules.toml
  const here = dirHere()
  const candidates = [
    join(here, '..', 'vacant', 'rules.toml'),
    join(here, '..', '..', 'vacant', 'rules.toml'),
    join(here, '..', '..', '..', 'vacant', 'rules.toml'),
  ]
  for (const path of candidates) {
    if (existsSync(path)) return path
  }
  // Fall through: return the most likely path so the binding emits a helpful error.
  return candidates[0]!
}
