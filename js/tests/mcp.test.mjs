// ABOUTME: End-to-end test for the `vacant mcp` stdio server over real JSON-RPC.
// ABOUTME: Spawns the built bin, runs the MCP handshake, and checks check_domains.

import { strict as assert } from 'node:assert'
import { spawn } from 'node:child_process'
import { execPath } from 'node:process'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'

const BIN = fileURLToPath(new URL('../dist/bin/vacant.cjs', import.meta.url))

// Drive the server through a session and return responses keyed by id.
function runSession(requests) {
  return new Promise((resolve, reject) => {
    const child = spawn(execPath, [BIN, 'mcp'], { stdio: ['pipe', 'pipe', 'pipe'] })
    let stdout = ''
    let stderr = ''
    child.stdout.setEncoding('utf8')
    child.stderr.setEncoding('utf8')
    child.stdout.on('data', (c) => (stdout += c))
    child.stderr.on('data', (c) => (stderr += c))
    child.on('error', reject)
    child.on('close', () => {
      assert.equal(stderr, '', `server logged to stderr: ${stderr}`)
      const responses = stdout
        .split(/\r?\n/)
        .filter((l) => l.trim().length > 0)
        .map((l) => JSON.parse(l))
      resolve(responses)
    })
    for (const req of requests) child.stdin.write(`${req}\n`)
    child.stdin.end() // EOF → server shuts down after replying
  })
}

const byId = (responses, id) => {
  const r = responses.find((x) => x.id === id)
  assert.ok(r, `no response with id ${id}`)
  return r
}

const INIT = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}'
const INITIALIZED = '{"jsonrpc":"2.0","method":"notifications/initialized"}'

test('lists and calls check_domains', async () => {
  const responses = await runSession([
    INIT,
    INITIALIZED,
    '{"jsonrpc":"2.0","id":2,"method":"tools/list"}',
    '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"check_domains","arguments":{"domains":["google.com","this-is-clearly-not-a-domain.example"]}}}',
  ])

  assert.equal(byId(responses, 1).result.serverInfo.name, 'vacant')
  assert.equal(byId(responses, 2).result.tools[0].name, 'check_domains')

  const entries = byId(responses, 3).result.structuredContent.result
  assert.equal(entries.length, 2)
  assert.equal(entries[0].domain, 'google.com')
  assert.ok(
    entries[0].status === 'registered' || entries[0].status === 'reserved',
    `google.com should be registered/reserved, got ${entries[0].status}`,
  )
  assert.equal(entries[1].domain, 'this-is-clearly-not-a-domain.example')
  assert.equal(entries[1].status, 'invalid')
})

test('skips blank and comment entries', async () => {
  const responses = await runSession([
    INIT,
    INITIALIZED,
    '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"check_domains","arguments":{"domains":["","  ","# a comment","google.com"]}}}',
  ])

  const entries = byId(responses, 2).result.structuredContent.result
  assert.equal(entries.length, 1)
  assert.equal(entries[0].domain, 'google.com')
})
