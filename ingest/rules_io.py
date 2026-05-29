# ABOUTME: Fast, format-preserving edits for rules/rules.toml.
# ABOUTME: Reads with stdlib tomllib and rewrites only changed lines, avoiding tomlkit's
# ABOUTME: minutes-long round-trip on the 16k-line file while keeping diffs minimal.
from __future__ import annotations

import re
import tomllib

_ZONE_HEADER = re.compile(r'^\[zone\.(?:"([^"]+)"|([A-Za-z0-9_-]+))\]$')
_KEY = re.compile(r"^([A-Za-z0-9_]+) = ")
_RDAP = re.compile(r"^rdap = ")
_BARE_KEY = re.compile(r"^[A-Za-z0-9_-]+$")


def load(text: str) -> dict:
    """Parse the file into plain data for decisions (fast; tomllib, no formatting)."""
    return tomllib.loads(text)


def _format_value(value: object) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    return '"' + str(value) + '"'


def zone_header(name: str) -> str:
    """The `[zone.<name>]` header as TOML writes it: bare key, or quoted if it has dots."""
    if _BARE_KEY.fullmatch(name):
        return f"[zone.{name}]"
    return f'[zone."{name}"]'


def apply_edits(
    text: str,
    *,
    meta: dict[str, object] | None = None,
    zone_rdap: dict[str, str] | None = None,
    new_zones: list[str] | None = None,
) -> str:
    """Return `text` with the requested edits applied, touching only changed lines.

    - `meta`: replace these keys' values in the `[meta]` table.
    - `zone_rdap`: set each zone's `rdap` (replacing any existing value, inserted
      right after the header so it leads the block).
    - `new_zones`: append these as empty `[zone.<name>]` blocks at the end.

    Passing no edits returns the input unchanged byte-for-byte.
    """
    meta = dict(meta or {})
    zone_rdap = dict(zone_rdap or {})
    new_zones = list(new_zones or [])

    out: list[str] = []
    in_meta = False
    drop_rdap = False  # inside a zone we're rewriting: drop its existing rdap line
    meta_remaining = dict(meta)  # keys not found in [meta] get appended on the way out
    last_meta_idx: int | None = None  # index in `out` just after the last [meta] key

    def flush_meta() -> None:
        nonlocal last_meta_idx
        if not meta_remaining:
            return
        at = last_meta_idx if last_meta_idx is not None else len(out)
        out[at:at] = [f"{k} = {_format_value(v)}" for k, v in meta_remaining.items()]
        meta_remaining.clear()

    for line in text.split("\n"):
        header = line.strip()
        if header.startswith("[") and header.endswith("]"):
            if in_meta:
                flush_meta()  # leaving [meta]: append any keys it didn't already have
            in_meta = header == "[meta]"
            drop_rdap = False
            out.append(line)
            match = _ZONE_HEADER.match(header)
            if match:
                name = match.group(1) if match.group(1) is not None else match.group(2)
                if name in zone_rdap:
                    out.append(f'rdap = "{zone_rdap[name]}"')
                    drop_rdap = True
            continue

        if in_meta:
            key = _KEY.match(line)
            if key:
                if key.group(1) in meta:
                    out.append(f"{key.group(1)} = {_format_value(meta[key.group(1)])}")
                    meta_remaining.pop(key.group(1), None)
                else:
                    out.append(line)
                last_meta_idx = len(out)
                continue

        if drop_rdap and _RDAP.match(line):
            continue  # superseded by the line inserted after the header

        out.append(line)

    if in_meta:
        flush_meta()  # [meta] was the final section

    if new_zones:
        if out and out[-1] == "":
            out.pop()  # the trailing blank stands in for the final newline
        for name in new_zones:
            out.append("")
            out.append(zone_header(name))
        out.append("")

    return "\n".join(out)
