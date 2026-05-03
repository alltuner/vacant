#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx>=0.28", "tomlkit>=0.14"]
# ///
# Note: tomlkit preserves comments/formatting but is slow (~minute on a 14k-line file).
# We pre-check `[meta]` with stdlib tomllib so a no-op dry-run is instant.
# ABOUTME: Refresh rules/rules.toml from the Public Suffix List ICANN section.
# ABOUTME: Run via `uv run ingest/psl.py [--dry-run] [--force]`. Bumps [meta] psl_version + psl_commit.
from __future__ import annotations

import argparse
import re
import sys
import tomllib
from datetime import datetime, timezone
from pathlib import Path

import httpx
import tomlkit
from tomlkit import TOMLDocument, table
from tomlkit.items import Table

URL = "https://publicsuffix.org/list/public_suffix_list.dat"
RULES = Path(__file__).resolve().parent.parent / "rules" / "rules.toml"

_VERSION_RE = re.compile(r"^//\s*VERSION:\s*(?P<v>\S+)")
_COMMIT_RE = re.compile(r"^//\s*COMMIT:\s*(?P<c>[0-9a-f]+)", re.IGNORECASE)


def fetch() -> str:
    response = httpx.get(URL, timeout=30.0, follow_redirects=True)
    response.raise_for_status()
    return response.text


def parse_header(text: str) -> tuple[str, str]:
    version = ""
    commit = ""
    for line in text.splitlines():
        if not version and (m := _VERSION_RE.match(line)):
            version = m.group("v")
        if not commit and (m := _COMMIT_RE.match(line)):
            commit = m.group("c")
        if version and commit:
            break
    return version, commit


def parse_icann(text: str) -> list[str]:
    in_section = False
    out: list[str] = []
    for line in text.splitlines():
        stripped = line.strip()
        if "===BEGIN ICANN DOMAINS===" in stripped:
            in_section = True
            continue
        if "===END ICANN DOMAINS===" in stripped:
            break
        if not in_section or not stripped or stripped.startswith("//"):
            continue
        if stripped.startswith(("*", "!")):
            continue
        out.append(stripped.lower())
    return out


def apply_suffixes(doc: TOMLDocument, suffixes: list[str]) -> list[str]:
    zones = doc.get("zone")
    if zones is None or not isinstance(zones, Table):
        zones = table()
        doc["zone"] = zones
    existing = set(zones.keys())
    added: list[str] = []
    for suffix in suffixes:
        if suffix in existing:
            continue
        zones[suffix] = table()
        added.append(suffix)
    return added


def update_meta(doc: TOMLDocument, version: str, commit: str) -> None:
    meta = doc.setdefault("meta", {})
    meta["psl_version"] = version
    meta["psl_commit"] = commit
    meta["psl_imported_at"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def already_current(doc: dict | TOMLDocument, version: str, commit: str) -> bool:
    meta = doc.get("meta")
    if meta is None:
        return False
    return (
        str(meta.get("psl_version", "")) == version
        and str(meta.get("psl_commit", "")) == commit
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="Show changes without writing")
    parser.add_argument("--force", action="store_true", help="Re-apply even if version unchanged")
    args = parser.parse_args()

    text = fetch()
    version, commit = parse_header(text)

    raw = RULES.read_text(encoding="utf-8")
    if not args.force and already_current(tomllib.loads(raw), version, commit):
        sys.stdout.write(f"already at PSL version {version} ({commit[:8]})\n")
        return 0

    doc = tomlkit.parse(raw)
    suffixes = parse_icann(text)
    added = apply_suffixes(doc, suffixes)
    for suffix in added[:20]:
        sys.stdout.write(f"+ {suffix}\n")
    if len(added) > 20:
        sys.stdout.write(f"  ... and {len(added) - 20} more\n")
    sys.stdout.write(
        f"{len(added)} zone(s) added (PSL {version}, commit {commit[:8]})"
        f"{'; dry-run, not writing' if args.dry_run else ''}\n"
    )
    if not args.dry_run:
        update_meta(doc, version, commit)
        RULES.write_text(tomlkit.dumps(doc), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
