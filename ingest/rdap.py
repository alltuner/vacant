#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx>=0.28", "tomlkit>=0.14"]
# ///
# ABOUTME: Refresh rules/rules.toml with RDAP service URLs from the IANA bootstrap.
# ABOUTME: With --probe, also discovers RDAP endpoints for TLDs the bootstrap omits.
from __future__ import annotations

import argparse
import asyncio
import socket
import sys
import tomllib
from datetime import datetime, timezone
from pathlib import Path

import httpx
import tomlkit
from tomlkit import TOMLDocument
from tomlkit.items import Table

URL = "https://data.iana.org/rdap/dns.json"
RULES = Path(__file__).resolve().parent.parent / "rules" / "rules.toml"

# The IANA bootstrap is opt-in, so many ccTLD registries that run RDAP aren't in
# it. Most follow the `rdap.nic.<tld>` convention. --probe tries these candidates
# and keeps one only if it actually answers an RDAP query for that TLD's own
# domain, so a wrong or catch-all endpoint is never recorded.
CANDIDATE_TEMPLATES = ("https://rdap.nic.{tld}", "https://rdap.{tld}")
# Short connect so the many candidate hosts that don't exist fail fast.
PROBE_TIMEOUT = httpx.Timeout(8.0, connect=4.0)


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


def missing_tlds(doc: TOMLDocument, mapping: dict[str, str]) -> list[str]:
    """Top-level zones with no RDAP endpoint, from the bootstrap or already set.

    Only TLDs are probed; multi-level suffixes (e.g. `co.uk`) inherit their
    parent TLD's endpoint through `apply_map`. IDN TLDs are skipped — the
    `rdap.nic.<punycode>` convention does not hold for them.
    """
    out: list[str] = []
    seen: set[str] = set()
    zones = doc.get("zone")
    if zones is None:
        return out
    for zone_name, zone_table in zones.items():
        if not isinstance(zone_table, Table) or "." in zone_name:
            continue
        tld = zone_name.lower()
        if tld in seen or tld.startswith("xn--"):
            continue
        seen.add(tld)
        if mapping.get(tld) is None and zone_table.get("rdap") is None:
            out.append(tld)
    return out


async def validate(client: httpx.AsyncClient, base: str, tld: str) -> str | None:
    """A candidate is valid only if it serves an RDAP domain object for the
    TLD's own `nic.<tld>` name — proof it's a real RDAP endpoint for that TLD."""
    try:
        response = await client.get(
            f"{base}/domain/nic.{tld}",
            headers={"Accept": "application/rdap+json"},
        )
    except httpx.HTTPError:
        return None
    if response.status_code != 200:
        return None
    try:
        data = response.json()
    except ValueError:
        return None
    if data.get("objectClassName") != "domain":
        return None
    ldh = str(data.get("ldhName", "")).lower().rstrip(".")
    return base if ldh.endswith(f".{tld}") else None


async def host_resolves(host: str) -> bool:
    """Cheap DNS pre-check. Most candidate hostnames don't exist, and skipping
    the HTTP/TLS attempt for those is what keeps the probe fast and bounded."""
    loop = asyncio.get_running_loop()
    try:
        await asyncio.wait_for(loop.getaddrinfo(host, 443, type=socket.SOCK_STREAM), timeout=3.0)
        return True
    except (asyncio.TimeoutError, OSError):
        return False


async def probe_one(
    client: httpx.AsyncClient, sem: asyncio.Semaphore, tld: str
) -> tuple[str, str | None]:
    async with sem:
        for template in CANDIDATE_TEMPLATES:
            base = template.format(tld=tld)
            host = base.split("://", 1)[1]
            if not await host_resolves(host):
                continue
            found = await validate(client, base, tld)
            if found is not None:
                return tld, found
    return tld, None


async def probe(tlds: list[str], concurrency: int, budget: float) -> tuple[dict[str, str], int]:
    """Probe every TLD, but never run longer than `budget` seconds. Returns the
    discovered endpoints and the count of probes still pending at the deadline
    (so most failing-to-resolve candidates can't stall the whole refresh)."""
    sem = asyncio.Semaphore(concurrency)
    discovered: dict[str, str] = {}
    async with httpx.AsyncClient(timeout=PROBE_TIMEOUT, follow_redirects=True) as client:
        tasks = [asyncio.create_task(probe_one(client, sem, tld)) for tld in tlds]
        done, pending = await asyncio.wait(tasks, timeout=budget)
        for task in pending:
            task.cancel()
        if pending:
            await asyncio.gather(*pending, return_exceptions=True)
        for task in done:
            tld, url = task.result()
            if url is not None:
                discovered[tld] = url
    return discovered, len(pending)


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
    parser.add_argument(
        "--probe",
        action="store_true",
        help="Discover RDAP endpoints for TLDs the IANA bootstrap omits",
    )
    parser.add_argument(
        "--probe-concurrency", type=int, default=16, help="Concurrent probes (default: 16)"
    )
    parser.add_argument(
        "--probe-budget",
        type=float,
        default=240.0,
        help="Max seconds to spend probing before giving up on stragglers (default: 240)",
    )
    args = parser.parse_args()

    payload = fetch()
    raw = RULES.read_text(encoding="utf-8")
    if not args.force and not args.probe and already_current(tomllib.loads(raw), payload):
        sys.stdout.write(f"already at publication {payload.get('publication', '?')}\n")
        return 0

    doc = tomlkit.parse(raw)
    mapping = build_map(payload)

    if args.probe:
        targets = missing_tlds(doc, mapping)
        sys.stdout.write(f"probing {len(targets)} TLD(s) without a bootstrap RDAP endpoint...\n")
        sys.stdout.flush()
        discovered, timed_out = asyncio.run(
            probe(targets, args.probe_concurrency, args.probe_budget)
        )
        for tld, url in sorted(discovered.items()):
            sys.stdout.write(f"  discovered .{tld}: {url}\n")
        note = f" ({timed_out} still pending at the {args.probe_budget:g}s deadline)" if timed_out else ""
        sys.stdout.write(f"{len(discovered)} endpoint(s) discovered by probing{note}\n")
        mapping.update(discovered)

    changes = apply_map(doc, mapping)
    for zone, old, new in changes:
        sys.stdout.write(f"{zone}: {old or '(unset)'} -> {new}\n")
    sys.stdout.write(
        f"{len(changes)} zone(s) updated"
        f"{'; dry-run, not writing' if args.dry_run else ''}\n"
    )
    if not args.dry_run:
        update_meta(doc, payload)
        if args.probe:
            doc["meta"]["rdap_probed_at"] = datetime.now(timezone.utc).strftime(
                "%Y-%m-%dT%H:%M:%SZ"
            )
        RULES.write_text(tomlkit.dumps(doc), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
