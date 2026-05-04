# rules

`rules.toml` is the canonical PSL + RDAP-derived data that all `vacant` packages embed.

## Why it lives here

Every publishable artifact in this monorepo (the `vacant` crate, the `vacant` Python wheel, future bindings) needs a self-contained copy of the rules: `cargo publish` and `maturin sdist` only package files inside the package directory, so `include_str!` / wheel-bundled paths cannot reach a sibling location at runtime. To avoid implying that any one consumer "owns" the data, the source of truth is at the repo root.

## Mirrors

The following files are **build-time mirrors** of `rules/rules.toml`. Do not edit them by hand:

- `crates/vacant/data/rules.toml` — embedded into the Rust binary via `include_str!`
- `python/vacant/rules.toml` — bundled into the Python wheel; loaded at runtime by `vacant.checker`
- `js/vacant/rules.toml` — bundled into the npm package; loaded at runtime by the napi-rs binding

`just sync-rules` overwrites every mirror from the canonical file. CI runs `just check-rules-sync`, which fails the build on any drift.

## Refreshing

The weekly `refresh-rules` workflow runs `ingest/psl.py` and `ingest/rdap.py`, both of which write to `rules/rules.toml`. The workflow then runs `just sync-rules` and opens a PR containing the canonical file plus all mirrors.

To refresh locally:

```bash
just ingest-psl
just ingest-rdap
just sync-rules
```
