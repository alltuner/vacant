// ABOUTME: Bin entry point — wired up by package.json `bin` field.
// ABOUTME: Just calls cli.main() and propagates the exit code.

import { main } from './cli.js'

main().then(
  (code) => {
    process.exit(code)
  },
  (err) => {
    process.stderr.write(`vacant: ${err instanceof Error ? err.message : String(err)}\n`)
    process.exit(1)
  },
)
