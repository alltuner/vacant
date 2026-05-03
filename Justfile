# vacant — common dev tasks. `just <target>` or `just` for the menu.

default:
    @just --list

# Build the release binary.
build:
    cargo build --release -p vacant

# Run the binary on stdin or args (after a fresh build).
run *args: build
    ./target/release/vacant {{args}}

# Run all rust tests, parallel via nextest if available.
test:
    @cargo nextest run --workspace 2>/dev/null || cargo test --workspace

# Format + clippy + rules drift check.
check: check-rules-sync
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings

# Refresh rules/rules.toml from the Public Suffix List (writes the canonical file).
ingest-psl *args:
    uv run ingest/psl.py {{args}}

# Refresh rules/rules.toml with RDAP bootstrap data (writes the canonical file).
ingest-rdap *args:
    uv run ingest/rdap.py {{args}}

# Mirror rules/rules.toml into every package's embedded copy.
sync-rules:
    cp rules/rules.toml crates/vacant/data/rules.toml
    cp rules/rules.toml python/vacant/rules.toml

# Fail if any embedded copy of rules.toml has drifted from the canonical.
check-rules-sync:
    @diff -q rules/rules.toml crates/vacant/data/rules.toml >/dev/null || (echo "crates/vacant/data/rules.toml drifted from rules/rules.toml — run 'just sync-rules'" >&2; exit 1)
    @diff -q rules/rules.toml python/vacant/rules.toml >/dev/null || (echo "python/vacant/rules.toml drifted from rules/rules.toml — run 'just sync-rules'" >&2; exit 1)

# Build the maturin extension into the local venv.
py-develop: sync-rules
    cd python && uv run --with maturin maturin develop --uv

# Format + lint + tests for the python package.
py-check: sync-rules
    cd python && uv run ruff format --check .
    cd python && uv run ruff check .
    cd python && uv run pytest

# Build a release wheel locally.
py-wheel: sync-rules
    cd python && uv run --with maturin maturin build --release

# Build a source distribution.
py-sdist: sync-rules
    cd python && uv run --with maturin maturin sdist

# Clean build outputs.
clean:
    cargo clean
