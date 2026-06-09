// ABOUTME: MCP stdio server exposing vacant's domain checks as one read-only tool.
// ABOUTME: Mirrors python/vacant/mcp.py — check_domains(domains, verify=false) over checkMany.

import { stdin } from 'node:process'

import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { z } from 'zod'

import pkg from '../package.json' with { type: 'json' }
import { checkMany } from './checker.js'

const MAX_DOMAINS = 500

const CHECK_DOMAINS_DESCRIPTION = `Check domain availability across any TLD via authoritative DNS.

Pass one name or many; returns one {domain, status} per input, in order.
status is one of: registered, available, unconfirmed, reserved, invalid, unknown.

'available' only ever appears when verify=true, which confirms undelegated names \
against the registry (RDAP). Without it, a name with no DNS delegation reports \
'unconfirmed' (probably free, but a held or expired domain looks the same to DNS). \
Set verify=true when the user asks whether a specific name is free; leave it false \
to cheaply screen a large list down to candidates.`

const domainStatus = z.object({ domain: z.string(), status: z.string() })

export async function serve(): Promise<void> {
  const server = new McpServer({ name: 'vacant', version: pkg.version })

  server.registerTool(
    'check_domains',
    {
      description: CHECK_DOMAINS_DESCRIPTION,
      inputSchema: {
        domains: z.array(z.string()).describe('Domain names to check. Blank entries and ones starting with "#" are skipped.'),
        verify: z
          .boolean()
          .default(false)
          .describe('Confirm undelegated names against the registry (RDAP) so they resolve to "available" or "registered" rather than "unconfirmed".'),
      },
      // MCP requires structured output to have an object root, so the per-input
      // list is nested under `result` — matching the Python and Rust servers.
      outputSchema: { result: z.array(domainStatus) },
    },
    ({ domains, verify }) => {
      const cleaned = domains
        .map((d) => d.trim())
        .filter((d) => d.length > 0 && !d.startsWith('#'))
        .slice(0, MAX_DOMAINS)
      const result = checkMany(cleaned, { verify }).map((r) => ({
        domain: r.domain || r.input,
        status: r.status,
      }))
      return {
        content: [{ type: 'text', text: JSON.stringify({ result }) }],
        structuredContent: { result },
      }
    },
  )

  const transport = new StdioServerTransport()
  await server.connect(transport)

  // The transport only watches stdin's 'data'; closing the server (and the
  // process) on EOF is up to us. Without this, bin.ts would exit immediately.
  await new Promise<void>((resolve) => {
    stdin.once('end', resolve)
    stdin.once('close', resolve)
  })
  await server.close()
}
