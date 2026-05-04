<h1 align="center">vacant</h1>

<p align="center">
  <strong>Fast domain availability checker.</strong><br>
  Asks authoritative TLD nameservers directly instead of WHOIS.
</p>

<p align="center">
  <a href="https://github.com/sponsors/alltuner">Sponsor</a>
</p>

<p align="center">
  <img src="https://img.shields.io/crates/v/vacant?color=5B2333" alt="crates.io">
  <img src="https://img.shields.io/pypi/v/vacant?color=5B2333" alt="PyPI">
  <img src="https://img.shields.io/npm/v/@alltuner/vacant?color=5B2333" alt="npm">
  <img src="https://img.shields.io/github/license/alltuner/vacant?color=5B2333" alt="License">
  <img src="https://img.shields.io/github/stars/alltuner/vacant?color=5B2333" alt="Stars">
</p>

---

## Packages

This repo is a monorepo. The same engine ships in three forms:

| Package | Path | Registry |
|---|---|---|
| `vacant` (Rust library + CLI) | [`crates/vacant`](crates/vacant) | [crates.io](https://crates.io/crates/vacant) |
| `vacant` (Python wheel) | [`python`](python) | [PyPI](https://pypi.org/project/vacant/) |
| `@alltuner/vacant` (npm package) | [`js`](js) | [npm](https://www.npmjs.com/package/@alltuner/vacant) |

See each package's README for usage. The Python wheel embeds the Rust engine via [PyO3](https://pyo3.rs), the npm package embeds it via [napi-rs](https://napi.rs), and all three share the on-disk SQLite cache with the CLI.

The [PSL](https://publicsuffix.org/) + RDAP-derived rules every package consumes live at [`rules/rules.toml`](rules/) — see the [`rules/` README](rules/README.md) for the source-of-truth and mirror policy.

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

## Support the project

vacant is an open source project built by [David Poblador i Garcia](https://davidpoblador.com/) through [All Tuner Labs](https://www.alltuner.com/).

If this project saved you a registrar lookup, consider supporting its development.

❤️ **Sponsor development**
https://github.com/sponsors/alltuner

☕ **One-time support**
https://buymeacoffee.com/alltuner

Your support helps fund the continued development of vacant and other open source developer tools such as [Factory Floor](https://github.com/alltuner/factoryfloor).

## License

[MIT](LICENSE)

---

<p align="center">
  Built by <a href="https://davidpoblador.com">David Poblador i Garcia</a> with the support of <a href="https://alltuner.com">All Tuner Labs</a>.<br>
  Made with ❤️ in Poblenou, Barcelona.
</p>
