# vacant

Fast domain availability checker. Asks authoritative TLD nameservers directly
instead of hammering WHOIS, with a built-in rules engine for things WHOIS won't
tell you (no 2-letter `.eu` domains, registry suffixes, malformed inputs, etc.).

```
$ vacant anthropic.com testdomain.com3d ab.eu
{"domain":"anthropic.com","status":"registered"}
{"domain":"testdomain.com3d","status":"invalid"}
{"domain":"ab.eu","status":"reserved"}
```

## Install

```bash
brew install alltuner/tap/vacant
# or
cargo install vacant
```

## Usage

```bash
# single domain
vacant example.com

# many at once (positional, or one per line on stdin)
vacant a.com b.net c.org
cat domains.txt | vacant

# pretty table output
vacant -o table example.com

# include zone, raw input, detail, cache hit
vacant --detail example.com

# only show available results
vacant --available a.com b.com c.com

# tighten timeout, raise concurrency for large batches
vacant --timeout 2 --concurrency 256 < big-list.txt
```

Exit codes: `0` on success, `2` if any result has `status=unknown` (transport
failure, ambiguous registry response).

## How it works

For each input:

1. **Rule layer** rejects malformed strings (empty labels, no TLD), unknown
   TLDs (not in the bundled Public Suffix List), per-zone violations
   (e.g. `.eu` minimum 3 characters), and registry suffixes used as inputs
   (`co.uk` is not a registrable name).
2. **Authoritative NS query** against the parent zone's nameservers, with
   `+norecurse`. NS delegation present → `registered`. NXDOMAIN → `available`.
3. **RDAP fallback** for registries that use compact denial of existence
   (Nominet `.uk`, several others) where step 2 returns NODATA.

Per-host nameserver cooldowns (5 min after a transport failure) and per-RDAP-host
throttling (2 concurrent + 100ms gap) keep things polite under heavy use.

## Bundled data

`data/rules.toml` ships baked into the binary at compile time. It carries:

- The Public Suffix List ICANN section (~6900 zones)
- IANA RDAP bootstrap URLs per TLD
- Hand-curated per-zone overrides (length limits, character classes, etc.)

CI refreshes it weekly via `ingest/psl.py` and `ingest/rdap.py` (PEP 723
inline-deps scripts; run with `uv run ingest/psl.py`). Override the bundled
file at runtime with `--rules path/to/rules.toml` or `VACANT_RULES=...`.

## Develop

```bash
just              # menu
just build        # cargo build --release
just test         # cargo nextest / cargo test
just check        # fmt + clippy
just ingest-psl   # refresh PSL data (writes data/rules.toml)
just ingest-rdap  # refresh RDAP bootstrap (writes data/rules.toml)
```

## Etiquette

You're querying TLD operators (Verisign for `.com`/`.net`, Nominet for `.uk`,
etc.) directly. Sporadic interactive use is invisible noise to them; sustained
high-rate scanning would be impolite. The built-in cache (1-day TTL by default)
and per-host backoff are there to keep you well-behaved automatically.

## License

MIT. See [LICENSE](LICENSE).
