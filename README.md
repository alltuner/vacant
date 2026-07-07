<p align="center">
  <img src="https://brand.alltuner.com/logos/vacant/horizontal.png" alt="vacant" width="500">
</p>

<p align="center">
  <strong>Fast domain availability checker.</strong><br>
  Asks authoritative TLD nameservers directly instead of WHOIS.
</p>

<p align="center">
  <a href="https://alltuner.com/sponsor">Sponsor</a>
</p>

<p align="center">
  <img src="https://img.shields.io/crates/v/vacant?color=5B2333" alt="crates.io">
  <img src="https://img.shields.io/pypi/v/vacant?color=5B2333" alt="PyPI">
  <img src="https://img.shields.io/npm/v/@alltuner/vacant?color=5B2333" alt="npm">
  <img src="https://img.shields.io/github/license/alltuner/vacant?color=5B2333" alt="License">
  <img src="https://img.shields.io/github/stars/alltuner/vacant?color=5B2333" alt="Stars">
</p>

<p align="center">
  <img src="https://vacant.alltuner.com/static/demo.gif" alt="vacant checking domain availability" width="800">
</p>

---

## Get Started

```bash
# Rust / CLI
brew install alltuner/tap/vacant
cargo install vacant

# Python
uv add vacant

# JavaScript / TypeScript
npm install @alltuner/vacant
```

## Try it without installing

The CLI also runs straight from any package runner — no install, no global state:

```bash
uvx vacant example.com                    # PyPI wheel via uv
pipx run vacant example.com               # PyPI wheel via pipx
npx -y @alltuner/vacant example.com       # npm package via npx
bunx @alltuner/vacant example.com         # npm package via bun
pnpm dlx @alltuner/vacant example.com     # npm package via pnpm
```

All variants share the same Rust engine, so results and flags are identical. The native binary (`brew` / `cargo`) is fastest to start; the runner variants are perfect for one-shots and CI.

Prefer a browser? [**vacant.alltuner.com**](https://vacant.alltuner.com) is a hosted web UI built on these packages — a separate app for checking one name or hundreds at once, no install. (The packages here are the engine, CLI, and libraries; that site is just one example of something built with them.)

## For agents

There's a ready-made agent skill at [`alltuner/skills`](https://github.com/alltuner/skills) so coding agents (Claude Code, etc.) can use `vacant` directly when checking domain availability:

```bash
npx skills add alltuner/skills --skill vacant
```

The skill wraps the CLI with usage hints, common patterns, exit-code semantics, and registry gotchas — see [`skills/vacant/SKILL.md`](https://github.com/alltuner/skills/blob/main/skills/vacant/SKILL.md).

Every package also ships a `vacant mcp` subcommand — a Model Context Protocol server over stdio exposing one read-only tool, `check_domains(domains, verify=false)`. Run it from whichever channel you've installed:

```bash
vacant mcp                                 # native binary (brew / cargo)
npx -y @alltuner/vacant mcp                # npm package
uvx --from 'vacant[mcp]' vacant mcp        # PyPI wheel (needs the mcp extra)
```

## Packages

This repo is a monorepo. The same engine ships in three forms:

| Package | Path | Registry |
|---|---|---|
| `vacant` (Rust library + CLI) | [`crates/vacant`](crates/vacant) | [crates.io](https://crates.io/crates/vacant) |
| `vacant` (Python wheel) | [`python`](python) | [PyPI](https://pypi.org/project/vacant/) |
| `@alltuner/vacant` (npm package) | [`js`](js) | [npm](https://www.npmjs.com/package/@alltuner/vacant) |

See each package's README for usage. The Python wheel embeds the Rust engine via [PyO3](https://pyo3.rs), the npm package embeds it via [napi-rs](https://napi.rs), and all three share the on-disk SQLite cache with the CLI.

The [PSL](https://publicsuffix.org/) + RDAP-derived rules every package consumes live at [`rules/rules.toml`](rules/) — see the [`rules/` README](rules/README.md) for the source-of-truth and mirror policy.

## Development

```bash
just                 # menu of all dev tasks
just build           # build the rust binary
just test            # run rust workspace tests
just py-develop      # build the python extension into a local venv
just py-check        # ruff + pytest
just js-develop      # build the napi-rs binding into js/
just js-check        # tsc + node:test smoke
```

## License

[MIT](LICENSE)

## Support the project

vacant is an open source project built by [David Poblador i Garcia](https://davidpoblador.com/) through [All Tuner Labs](https://www.alltuner.com/).

If this project was useful to you, [consider supporting its development](https://alltuner.com/sponsor).

---

<p align="center">
  Built by <a href="https://davidpoblador.com">David Poblador i Garcia</a> with the support of <a href="https://alltuner.com">All Tuner Labs</a>.<br>
  Made with ❤️ in Poblenou, Barcelona.
</p>
