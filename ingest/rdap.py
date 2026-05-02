#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx>=0.28", "tomlkit>=0.14"]
# ///
# ABOUTME: Refresh data/rules.toml with RDAP service URLs from the IANA bootstrap.
# ABOUTME: Run via `uv run ingest/rdap.py [--dry-run] [--force]`. Bumps [meta] rdap_publication.
from __future__ import annotations

import argparse
import sys
import tomllib
from datetime import datetime, timezone
from pathlib import Path

import httpx
import tomlkit
from tomlkit import TOMLDocument
from tomlkit.items import Table

URL = "https://data.iana.org/rdap/dns.json"
RULES = Path(__file__).resolve().parent.parent / "data" / "rules.toml"


def fetch() -> dict:
    response = httpx.get(URL, timeout=30.0, follow_redirects=True)
    response.raise_for_status()
    return response.json()


def build_map(payload: dict) -> dict[str, str]:
    out: dict[str, str] = {}
    for entry in payload.get("services", []):
        if len(entry) != 2:
            continue
        tlds, urls = entry
        if not urls:
            continue
        chosen = next((u for u in urls if u.startswith("https://")), urls[0])
        chosen = chosen.rstrip("/")
        for tld in tlds:
            out[tld.lower()] = chosen
    return out


def apply_map(doc: TOMLDocument, mapping: dict[str, str]) -> list[tuple[str, str | None, str]]:
    changes: list[tuple[str, str | None, str]] = []
    zones = doc.get("zone")
    if zones is None:
        return changes
    for zone_name, zone_table in zones.items():
        if not isinstance(zone_table, Table):
            continue
        tld = zone_name.lower().split(".")[-1]
        target = mapping.get(tld)
        if target is None:
            continue
        current = zone_table.get("rdap")
        current_str = str(current) if current is not None else None
        if current_str == target:
            continue
        zone_table["rdap"] = target
        changes.append((zone_name, current_str, target))
    return changes


def update_meta(doc: TOMLDocument, payload: dict) -> None:
    meta = doc.setdefault("meta", {})
    meta["rdap_publication"] = payload.get("publication", "")
    meta["rdap_imported_at"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def already_current(doc: dict | TOMLDocument, payload: dict) -> bool:
    meta = doc.get("meta")
    if meta is None:
        return False
    return str(meta.get("rdap_publication", "")) == payload.get("publication", "__missing__")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="Show changes without writing")
    parser.add_argument("--force", action="store_true", help="Re-apply even if publication unchanged")
    args = parser.parse_args()

    payload = fetch()
    raw = RULES.read_text(encoding="utf-8")
    if not args.force and already_current(tomllib.loads(raw), payload):
        sys.stdout.write(f"already at publication {payload.get('publication', '?')}\n")
        return 0

    doc = tomlkit.parse(raw)
    mapping = build_map(payload)
    changes = apply_map(doc, mapping)
    for zone, old, new in changes:
        sys.stdout.write(f"{zone}: {old or '(unset)'} -> {new}\n")
    sys.stdout.write(
        f"{len(changes)} zone(s) updated"
        f"{'; dry-run, not writing' if args.dry_run else ''}\n"
    )
    if not args.dry_run:
        update_meta(doc, payload)
        RULES.write_text(tomlkit.dumps(doc), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
