// ABOUTME: Smoke test for the napi-rs binding — calls checkMany on google.com.
// ABOUTME: Mirrors python/tests/test_smoke.py. Loads the built dist bundle.

import { strict as assert } from 'node:assert'
import { test } from 'node:test'

import { Status, check, checkMany } from '../dist/index.js'

test('checkMany preserves input order and classifies', () => {
  const inputs = ['google.com', 'this-is-clearly-not-a-domain.example', '']
  const results = checkMany(inputs)
  assert.deepEqual(
    results.map((r) => r.input),
    inputs,
  )
  assert.ok(
    results[0].status === Status.REGISTERED || results[0].status === Status.RESERVED,
    `expected google.com registered/reserved, got ${results[0].status}`,
  )
  assert.equal(results[1].status, Status.INVALID)
  assert.equal(results[2].status, Status.INVALID)
})

test('check on a single invalid input', () => {
  const r = check('nope')
  assert.equal(r.status, Status.INVALID)
})
