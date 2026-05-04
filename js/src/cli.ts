// ABOUTME: vacant CLI entry point — mirrors python/vacant/cli.py.
// ABOUTME: Positional domains or stdin; -o jsonl|text; --concurrency; --timeout.

import { stdin } from 'node:process'

import { Status, checkMany, type Result } from './checker.js'

interface ParsedArgs {
  domains: string[]
  output: 'jsonl' | 'text'
  concurrency: number
  timeout: number
  showHelp: boolean
}

const HELP = `usage: vacant [-h] [-o {jsonl,text}] [--concurrency N] [--timeout SECONDS] [domains ...]

Check domain availability via authoritative DNS.

positional arguments:
  domains               Domains to check; reads stdin if empty.

options:
  -h, --help            show this help message and exit
  -o, --output {jsonl,text}
                        Output format (default: jsonl).
  --concurrency N       (default: 64)
  --timeout SECONDS     (default: 4.0)
`

function parseArgs(argv: string[]): ParsedArgs {
  const out: ParsedArgs = {
    domains: [],
    output: 'jsonl',
    concurrency: 64,
    timeout: 4.0,
    showHelp: false,
  }
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]!
    if (arg === '-h' || arg === '--help') {
      out.showHelp = true
    } else if (arg === '-o' || arg === '--output') {
      const value = argv[++i]
      if (value !== 'jsonl' && value !== 'text') {
        throw new Error(`--output must be 'jsonl' or 'text', got ${value ?? '(missing)'}`)
      }
      out.output = value
    } else if (arg.startsWith('--output=')) {
      const value = arg.slice('--output='.length)
      if (value !== 'jsonl' && value !== 'text') {
        throw new Error(`--output must be 'jsonl' or 'text', got ${value}`)
      }
      out.output = value
    } else if (arg === '--concurrency') {
      const value = argv[++i]
      const n = Number.parseInt(value ?? '', 10)
      if (!Number.isFinite(n) || n <= 0) throw new Error(`--concurrency requires a positive integer, got ${value ?? '(missing)'}`)
      out.concurrency = n
    } else if (arg.startsWith('--concurrency=')) {
      const n = Number.parseInt(arg.slice('--concurrency='.length), 10)
      if (!Number.isFinite(n) || n <= 0) throw new Error(`--concurrency requires a positive integer`)
      out.concurrency = n
    } else if (arg === '--timeout') {
      const value = argv[++i]
      const n = Number.parseFloat(value ?? '')
      if (!Number.isFinite(n) || n <= 0) throw new Error(`--timeout requires a positive number, got ${value ?? '(missing)'}`)
      out.timeout = n
    } else if (arg.startsWith('--timeout=')) {
      const n = Number.parseFloat(arg.slice('--timeout='.length))
      if (!Number.isFinite(n) || n <= 0) throw new Error(`--timeout requires a positive number`)
      out.timeout = n
    } else if (arg.startsWith('-')) {
      throw new Error(`unknown option: ${arg}`)
    } else {
      out.domains.push(arg)
    }
  }
  return out
}

async function readStdin(): Promise<string[]> {
  if (stdin.isTTY) return []
  let data = ''
  stdin.setEncoding('utf8')
  for await (const chunk of stdin) data += chunk
  return data
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith('#'))
}

function emit(results: Result[], output: 'jsonl' | 'text'): void {
  for (const r of results) {
    if (output === 'jsonl') {
      process.stdout.write(
        JSON.stringify({ domain: r.domain || r.input, status: r.status }) + '\n',
      )
    } else {
      process.stdout.write((r.domain || r.input) + '\n')
    }
  }
}

export async function main(argv: string[] = process.argv.slice(2)): Promise<number> {
  let parsed: ParsedArgs
  try {
    parsed = parseArgs(argv)
  } catch (err) {
    process.stderr.write(`vacant: ${(err as Error).message}\n`)
    process.stderr.write(HELP)
    return 2
  }

  if (parsed.showHelp) {
    process.stdout.write(HELP)
    return 0
  }

  const inputs = parsed.domains.length > 0 ? parsed.domains : await readStdin()
  if (inputs.length === 0) {
    process.stderr.write(HELP)
    return 2
  }

  const results = checkMany(inputs, {
    concurrency: parsed.concurrency,
    timeout: parsed.timeout,
  })
  emit(results, parsed.output)

  return results.some((r) => r.status === Status.UNKNOWN) ? 2 : 0
}
