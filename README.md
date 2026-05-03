# vacant

Fast domain availability checker. Asks authoritative TLD nameservers directly instead of WHOIS.

This repository hosts the engine and its language bindings:

| Package | Path | Registry |
|---|---|---|
| `vacant` (Rust library + CLI) | [`crates/vacant`](crates/vacant) | [crates.io](https://crates.io/crates/vacant) |
| `vacant` (Python wheel) | [`python`](python) | [PyPI](https://pypi.org/project/vacant/) |

See each package's README for usage. The Python wheel embeds the Rust engine via PyO3 and shares the on-disk SQLite cache with the CLI.

The PSL + RDAP-derived rules every package consumes live at [`rules/rules.toml`](rules/) — see the [`rules/` README](rules/README.md) for the source-of-truth and mirror policy.

## Develop

```bash
just                 # menu of all dev tasks
just build           # build the rust binary
just test            # run rust workspace tests
just py-develop      # build the python extension into a local venv
just py-check        # ruff + pytest
```

## License

MIT — see [`LICENSE`](LICENSE).
