// ABOUTME: tsup build config — emits dual ESM+CJS, plus a Node-shebanged CLI bin.
// ABOUTME: Native bindings (index.cjs / index.d.ts at repo root) are loaded at runtime, not bundled.

import { defineConfig } from 'tsup'

export default defineConfig([
  {
    entry: ['src/index.ts'],
    format: ['esm', 'cjs'],
    outDir: 'dist',
    dts: true,
    clean: true,
    splitting: false,
    sourcemap: false,
    target: 'node18',
    outExtension: ({ format }) => ({ js: format === 'cjs' ? '.cjs' : '.js' }),
    esbuildOptions(options) {
      // import.meta.url is only reachable on the ESM branch (guarded by typeof __dirname).
      // CJS pass complains anyway; suppress.
      options.logOverride = { ...(options.logOverride ?? {}), 'empty-import-meta': 'silent' }
    },
  },
  {
    entry: { 'bin/vacant': 'src/bin.ts' },
    format: ['cjs'],
    outDir: 'dist',
    dts: false,
    clean: false,
    splitting: false,
    sourcemap: false,
    target: 'node18',
    banner: { js: '#!/usr/bin/env node' },
    outExtension: () => ({ js: '.cjs' }),
    esbuildOptions(options) {
      options.logOverride = { ...(options.logOverride ?? {}), 'empty-import-meta': 'silent' }
    },
  },
])
