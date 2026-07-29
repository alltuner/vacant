# ABOUTME: Tests for the format-preserving rules.toml editor.
# ABOUTME: Covers the forbidden_labels writer plus idempotency and no-op guarantees.
from __future__ import annotations

import tomllib

import rules_io

BASE = """[meta]
psl_version = "x"

[zone.com]
rdap = "https://rdap.verisign.com/com/v1"

[zone."co.uk"]
rdap = "https://rdap.nominet.uk/uk"

[zone."org.uk"]
rdap = "https://rdap.nominet.uk/uk"
forbidden_labels = ["old"]

[zone."москва"]
rdap = "https://rdap.example/moscow"

[zone."bodø.no"]
"""


def test_noop_returns_input_byte_for_byte():
    assert rules_io.apply_edits(BASE) == BASE


def test_zone_forbidden_inserts_when_absent():
    out = rules_io.apply_edits(BASE, zone_forbidden={"co.uk": ["ac", "gov"]})
    data = tomllib.loads(out)
    assert data["zone"]["co.uk"]["forbidden_labels"] == ["ac", "gov"]
    # the zone's existing rdap is preserved
    assert data["zone"]["co.uk"]["rdap"] == "https://rdap.nominet.uk/uk"


def test_zone_forbidden_replaces_existing():
    out = rules_io.apply_edits(BASE, zone_forbidden={"org.uk": ["gov", "police"]})
    data = tomllib.loads(out)
    assert data["zone"]["org.uk"]["forbidden_labels"] == ["gov", "police"]
    # the old value is dropped, not duplicated
    assert out.count("forbidden_labels = ") == 1


def test_untouched_zone_is_left_alone():
    out = rules_io.apply_edits(BASE, zone_forbidden={"co.uk": ["gov"]})
    data = tomllib.loads(out)
    assert data["zone"]["com"]["rdap"] == "https://rdap.verisign.com/com/v1"
    assert "forbidden_labels" not in data["zone"]["com"]


def test_zone_forbidden_is_idempotent():
    edit = {"co.uk": ["ac", "gov"], "org.uk": ["gov", "police"]}
    once = rules_io.apply_edits(BASE, zone_forbidden=edit)
    twice = rules_io.apply_edits(once, zone_forbidden=edit)
    assert once == twice


def test_rdap_and_forbidden_on_same_zone():
    out = rules_io.apply_edits(
        BASE,
        zone_rdap={"co.uk": "https://example/rdap"},
        zone_forbidden={"co.uk": ["gov"]},
    )
    data = tomllib.loads(out)
    assert data["zone"]["co.uk"]["rdap"] == "https://example/rdap"
    assert data["zone"]["co.uk"]["forbidden_labels"] == ["gov"]


def test_new_meta_keys_are_appended():
    out = rules_io.apply_edits(BASE, meta={"forbidden_x_count": 3})
    data = tomllib.loads(out)
    assert data["meta"]["forbidden_x_count"] == 3


def test_zone_rename_keeps_the_block_contents():
    out = rules_io.apply_edits(BASE, zone_renames={"москва": "xn--80adxhks"})
    data = tomllib.loads(out)
    assert "москва" not in data["zone"]
    assert data["zone"]["xn--80adxhks"]["rdap"] == "https://rdap.example/moscow"


def test_zone_rename_emits_a_bare_key_when_it_can():
    out = rules_io.apply_edits(BASE, zone_renames={"москва": "xn--80adxhks"})
    assert "[zone.xn--80adxhks]" in out


def test_zone_rename_quotes_a_dotted_name():
    out = rules_io.apply_edits(BASE, zone_renames={"bodø.no": "xn--bod-2na.no"})
    assert '[zone."xn--bod-2na.no"]' in out
    assert "xn--bod-2na.no" in tomllib.loads(out)["zone"]


def test_zone_rename_composes_with_rdap():
    out = rules_io.apply_edits(
        BASE,
        zone_renames={"москва": "xn--80adxhks"},
        zone_rdap={"москва": "https://rdap.example/new"},
    )
    data = tomllib.loads(out)
    assert data["zone"]["xn--80adxhks"]["rdap"] == "https://rdap.example/new"
    assert out.count("https://rdap.example/moscow") == 0


def test_format_label_list():
    assert rules_io._format_label_list(["a", "b"]) == '["a", "b"]'
    assert rules_io._format_label_list([]) == "[]"
