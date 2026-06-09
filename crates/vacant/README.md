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

> **Prefer a browser?** [vacant.alltuner.com](https://vacant.alltuner.com) is a hosted web UI built on vacant — a separate app that uses this crate, not something it installs.

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

# confirm undelegated names against the registry (RDAP)
vacant --verify example.com

# only show available results (implies you want the truth — pair with --verify)
vacant --verify --available a.com b.com c.com

# tighten timeout, raise concurrency for large batches
vacant --timeout 2 --concurrency 256 < big-list.txt
```

A name with no delegation reports `unconfirmed`, not `available`: the DNS
answer for a free name and a held/suspended/pending-delete one is identical
(both NXDOMAIN), so "not delegated" isn't the same claim as "registrable". Pass
`--verify` to confirm those names against the registry's RDAP endpoint, which
promotes them to `available` (RDAP 404) or `registered` (RDAP 200, e.g. a domain
on hold). `available` therefore only ever means RDAP-confirmed registrable.

One caveat: RDAP-confirmed registrable is not a registrability guarantee. New
gTLD registries — especially community/city zones like `.barcelona`, `.cat`,
`.amsterdam`, `.berlin` — keep reservation and premium lists that aren't visible
via DNS, WHOIS, or RDAP, so a name can read `available` here yet still be refused
at checkout (e.g. `radio.barcelona`). Tracked for an in-engine fix in
[#26](https://github.com/alltuner/vacant/issues/26).

Exit codes: `0` on success, `2` if any result has `status=unknown` (transport
failure, ambiguous registry response). `unconfirmed` is a normal result and does
not affect the exit code.

## How it works

For each input:

1. **Rule layer** rejects malformed strings (empty labels, no TLD), unknown
   TLDs (not in the bundled Public Suffix List), per-zone violations
   (e.g. `.eu` minimum 3 characters), and registry suffixes used as inputs
   (`co.uk` is not a registrable name).
2. **Authoritative NS query** against the parent zone's nameservers, with
   `+norecurse`. NS delegation present → `registered` (a hard fact). NXDOMAIN or
   NODATA → `unconfirmed`: not delegated, probably free, but not verified.
3. **RDAP confirmation** (only with `--verify`) for the `unconfirmed` names, when
   the zone has an RDAP endpoint: 404 → `available`, 200 → `registered`,
   inconclusive → stays `unconfirmed`. Without `--verify` no RDAP traffic happens
   at all, so delegated (taken) names cost zero RDAP either way.

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
