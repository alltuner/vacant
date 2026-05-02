# vacant — common dev tasks. `just <target>` or `just` for the menu.

default:
    @just --list

# Build the release binary.
build:
    cargo build --release

# Run the binary on stdin or args (after a fresh build).
run *args: build
    ./target/release/vacant {{args}}

# Run all rust tests, parallel via nextest if available.
test:
    @cargo nextest run 2>/dev/null || cargo test

# Format + clippy.
check:
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings

# Refresh data/rules.toml from the Public Suffix List (writes the file).
ingest-psl *args:
    uv run ingest/psl.py {{args}}

# Refresh data/rules.toml with RDAP bootstrap data (writes the file).
ingest-rdap *args:
    uv run ingest/rdap.py {{args}}

# Build the PyPI placeholder wheel/sdist (does not publish).
build-placeholder:
    cd pypi-placeholder && uv build

# Clean rust + python build outputs.
clean:
    cargo clean
    rm -rf pypi-placeholder/dist pypi-placeholder/*.egg-info
