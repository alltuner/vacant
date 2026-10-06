// ABOUTME: tsdown build config — emits dual ESM+CJS, plus a Node-shebanged CLI bin; `tsc -p tsconfig.build.json` emits the .d.ts.
// ABOUTME: Native bindings (index.cjs / index.d.ts at repo root) are loaded at runtime, not bundled.

import { defineConfig } from 'tsdown'

export default defineConfig([
  {
    entry: ['src/index.ts'],
    format: ['esm', 'cjs'],
    outDir: 'dist',
    // rolldown-plugin-dts needs the TypeScript JS compiler API, which TypeScript 7 does not ship.
    dts: false,
    clean: true,
    sourcemap: false,
    target: 'node18',
    // `.js` for ESM (the package is `type: module`), `.cjs` for CJS.
    fixedExtension: false,
    // One self-contained file per entry and format, no shared chunks.
    outputOptions: { codeSplitting: false },
  },
  {
    entry: { 'bin/vacant': 'src/bin.ts' },
    format: ['cjs'],
    outDir: 'dist',
    dts: false,
    clean: false,
    sourcemap: false,
    target: 'node18',
    banner: '#!/usr/bin/env node',
    // Inline the lazily imported MCP server instead of emitting it as a separate chunk.
    outputOptions: { codeSplitting: false },
  },
])
