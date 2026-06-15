#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# ///
# ABOUTME: Refresh rules/rules.toml forbidden_labels (labels reserved by registry policy).
# ABOUTME: Run via `uv run ingest/forbidden.py [--dry-run] [--force]`. One handler per registry.
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Callable

import rules_io

RULES = Path(__file__).resolve().parent.parent / "rules" / "rules.toml"

# A handler's output collapsing to nothing, or shrinking past this fraction of
# what it last produced, is treated as a broken source rather than a real edit.
SHRINK_GUARD_RATIO = 0.5


@dataclass(frozen=True)
class Result:
    """One registry's verdict: the forbidden labels it owns, keyed by zone."""

    registry: str
    zones: dict[str, list[str]]

    def label_count(self) -> int:
        return sum(len(labels) for labels in self.zones.values())

    def content_hash(self) -> str:
        canonical = json.dumps(
            {zone: sorted(labels) for zone, labels in self.zones.items()},
            sort_keys=True,
        )
        return hashlib.sha256(canonical.encode("utf-8")).hexdigest()[:16]


# Nominet "Rules of Registration" (current revision 20.03.2024), Rule 5.5 + 5.6:
# https://www.nominet.uk/uk-domains/policies/
# Rule 5.5 forbids the existing-SLD labels, Rule 5.6 forbids "com" and "uk", both
# as a third-level label within the co.uk / me.uk / org.uk / net.uk zones. The set
# is closed and changes on a multi-year cadence, so it is pinned here with a manual
# re-check cadence rather than scraped from the (URL-unstable) policy PDF.
_RULE_5_5 = (
    "ac",
    "co",
    "gov",
    "ltd",
    "me",
    "mil",
    "mod",
    "net",
    "nhs",
    "nic",
    "org",
    "plc",
    "police",
    "sch",
)
_RULE_5_6 = ("com", "uk")
_NOMINET_LABELS = sorted(_RULE_5_5 + _RULE_5_6)
_NOMINET_ZONES = ("co.uk", "me.uk", "org.uk", "net.uk")


def nominet() -> Result:
    return Result("nominet", {zone: list(_NOMINET_LABELS) for zone in _NOMINET_ZONES})


# Adding a registry = adding a handler above and one entry here. Nothing else.
HANDLERS: tuple[Callable[[], Result], ...] = (nominet,)


def guard(result: Result, meta: dict) -> str | None:
    """Reject output that looks like a broken source. Returns an error or None."""
    previous = meta.get(f"forbidden_{result.registry}_count")
    if not isinstance(previous, int) or previous <= 0:
        return None  # nothing to compare against (first run)
    current = result.label_count()
    if current == 0:
        return f"{result.registry}: produced 0 labels, previously {previous}"
    if current < previous * SHRINK_GUARD_RATIO:
        return f"{result.registry}: produced {current} labels, previously {previous} (>50% drop)"
    return None


def compute_changes(
    zones: dict, desired: dict[str, list[str]]
) -> list[tuple[str, list[str] | None, list[str]]]:
    """Zones whose forbidden_labels should change, as (zone, old, new)."""
    changes: list[tuple[str, list[str] | None, list[str]]] = []
    for zone_name, labels in sorted(desired.items()):
        current = zones.get(zone_name, {}).get("forbidden_labels")
        if current == labels:
            continue
        changes.append((zone_name, current, labels))
    return changes


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="Show changes without writing")
    parser.add_argument("--force", action="store_true", help="Write even if nothing changed")
    args = parser.parse_args()

    raw = RULES.read_text(encoding="utf-8")
    data = rules_io.load(raw)
    meta = data.get("meta", {})
    zones = data.get("zone", {})

    results = [handler() for handler in HANDLERS]

    failures = [msg for r in results if (msg := guard(r, meta)) is not None]
    if failures:
        for msg in failures:
            sys.stderr.write(f"refusing to write — {msg}\n")
        sys.stderr.write("source looks broken; investigate before refreshing\n")
        return 1

    missing = [f"{r.registry}:{zone}" for r in results for zone in r.zones if zone not in zones]
    if missing:
        sys.stderr.write(f"target zone(s) not in rules.toml: {', '.join(missing)}\n")
        return 1

    desired: dict[str, list[str]] = {}
    for r in results:
        desired.update(r.zones)

    changes = compute_changes(zones, desired)
    for zone, old, new in changes:
        sys.stdout.write(f"{zone}: {old if old is not None else '(unset)'} -> {new}\n")
    sys.stdout.write(
        f"{len(changes)} zone(s) updated{'; dry-run, not writing' if args.dry_run else ''}\n"
    )

    if not changes and not args.force:
        return 0
    if args.dry_run:
        return 0

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    meta_updates: dict[str, object] = {}
    for r in results:
        meta_updates[f"forbidden_{r.registry}_count"] = r.label_count()
        meta_updates[f"forbidden_{r.registry}_hash"] = r.content_hash()
        meta_updates[f"forbidden_{r.registry}_fetched_at"] = now
    text = rules_io.apply_edits(
        raw,
        meta=meta_updates,
        zone_forbidden={zone: new for zone, _, new in changes},
    )
    RULES.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
