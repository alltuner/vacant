# @alltuner/vacant — domain availability via authoritative DNS

[![npm](https://img.shields.io/npm/v/@alltuner/vacant.svg)](https://www.npmjs.com/package/@alltuner/vacant)
[![PyPI](https://img.shields.io/pypi/v/vacant.svg)](https://pypi.org/project/vacant/)
[![crates.io](https://img.shields.io/crates/v/vacant.svg)](https://crates.io/crates/vacant)

JavaScript / TypeScript bindings for the [vacant](https://github.com/alltuner/vacant) Rust engine. Asks the authoritative TLD nameservers directly instead of WHOIS — fast, no rate limits, no waiting.

The package ships the same Rust engine compiled in via [napi-rs](https://napi.rs), with a small TypeScript facade and a `vacant` CLI entry point. Lockstep-versioned with the `vacant` crate: `@alltuner/vacant 0.4.x` wraps `vacant 0.4.x` (Rust) exactly.

## Install

Pick the path that matches how you'll use it:

### CLI

```bash
brew install alltuner/tap/vacant   # macOS, Linux — native Rust binary
cargo install vacant               # any platform with a Rust toolchain
npx -y @alltuner/vacant google.com # one-shot, no install (Node wheel)
```

The brew / cargo paths give you the native Rust binary (instant startup, ideal for daily use). `npx` runs the Node package — convenient when you don't want a global install, slightly slower to start because it boots Node.

### Library

```bash
npm install @alltuner/vacant
# or
pnpm add @alltuner/vacant
```

```ts
import { checkMany, Status } from '@alltuner/vacant'

const results = checkMany(['example.com', 'anthropic.com', 'totally-made-up-zxqv.cat'])
for (const r of results) {
  console.log(r.domain, r.status, r.detail)
}
```

The on-disk SQLite cache is shared with the Rust CLI — runs against the same `~/.cache/vacant/results.db`, so the brew binary and a Node script see each other's results.

```ts
import { DiskCache, checkMany } from '@alltuner/vacant'

const cache = new DiskCache() // default ~/.cache/vacant/results.db
const results = checkMany(['example.com'], { cache })
```

## How it works

`checkMany` calls into the Rust engine via napi-rs. The engine:

1. Normalizes the input.
2. Looks up cache; returns hits immediately.
3. Runs a per-zone precheck (length, charset, reserved labels) from the bundled `rules.toml`.
4. For inputs that pass, asks the parent zone's NS directly. NXDOMAIN → available; delegation → registered; ambiguous answers fall back to RDAP.

Cache shape, rules format, and verdict semantics are all the engine's — see [alltuner/vacant](https://github.com/alltuner/vacant) for the source of truth.

## Supported platforms

Prebuilt binaries ship for:

- `darwin-arm64` (Apple silicon)
- `darwin-x64` (Intel macs)
- `linux-arm64-gnu`
- `linux-x64-gnu`

npm picks the right one via `optionalDependencies` at install time. Other platforms aren't supported in v1; build from source via the Rust crate (`cargo install vacant`) if you need them.

## Develop

The Node package lives in the [alltuner/vacant](https://github.com/alltuner/vacant) monorepo alongside the Rust engine, so dev commands run from the repo root:

```bash
just                 # menu
just js-develop      # build the napi-rs extension into js/
just js-check        # tsc + node:test smoke
just js-pack         # produce a publishable tarball
```

## License

MIT — see [`../LICENSE`](../LICENSE).
