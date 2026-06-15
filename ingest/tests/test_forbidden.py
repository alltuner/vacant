# ABOUTME: Tests for the registry-policy forbidden-label ingest tool.
# ABOUTME: Covers the Nominet handler, sanity guard, change detection, and the write path.
from __future__ import annotations

import sys
import tomllib

import forbidden


def test_nominet_covers_the_four_zones():
    result = forbidden.nominet()
    assert set(result.zones) == {"co.uk", "me.uk", "org.uk", "net.uk"}
    for labels in result.zones.values():
        assert "gov" in labels
        assert labels == sorted(labels)
        assert len(labels) == 16


def test_label_count_and_hash_are_stable():
    a = forbidden.nominet()
    b = forbidden.nominet()
    assert a.label_count() == 64  # 16 labels across 4 zones
    assert a.content_hash() == b.content_hash()


def test_guard_passes_on_first_run():
    assert forbidden.guard(forbidden.nominet(), {}) is None


def test_guard_passes_when_count_is_stable():
    result = forbidden.nominet()
    assert forbidden.guard(result, {"forbidden_nominet_count": result.label_count()}) is None


def test_guard_rejects_collapse_to_zero():
    empty = forbidden.Result("nominet", {"co.uk": []})
    assert forbidden.guard(empty, {"forbidden_nominet_count": 64}) is not None


def test_guard_rejects_large_shrink():
    shrunk = forbidden.Result("nominet", {"co.uk": ["gov"]})  # 1 vs prior 64
    assert forbidden.guard(shrunk, {"forbidden_nominet_count": 64}) is not None


def test_compute_changes_detects_unset_and_skips_equal():
    desired = {"co.uk": ["gov"]}
    assert forbidden.compute_changes({}, desired) == [("co.uk", None, ["gov"])]
    unchanged = forbidden.compute_changes({"co.uk": {"forbidden_labels": ["gov"]}}, desired)
    assert unchanged == []


def _fixture(rules):
    rules.write_text(
        '[meta]\npsl_version = "x"\n\n'
        '[zone."co.uk"]\n[zone."me.uk"]\n[zone."org.uk"]\n[zone."net.uk"]\n',
        encoding="utf-8",
    )


def test_main_populates_then_is_idempotent(tmp_path, monkeypatch, capsys):
    rules = tmp_path / "rules.toml"
    _fixture(rules)
    monkeypatch.setattr(forbidden, "RULES", rules)
    monkeypatch.setattr(sys, "argv", ["forbidden.py"])

    assert forbidden.main() == 0
    data = tomllib.loads(rules.read_text())
    assert "gov" in data["zone"]["co.uk"]["forbidden_labels"]
    assert data["meta"]["forbidden_nominet_count"] == 64

    before = rules.read_text()
    capsys.readouterr()  # drop the first run's output
    assert forbidden.main() == 0
    assert rules.read_text() == before  # nothing rewritten
    assert "0 zone(s) updated" in capsys.readouterr().out


def test_main_refuses_when_target_zone_missing(tmp_path, monkeypatch):
    rules = tmp_path / "rules.toml"
    rules.write_text('[meta]\n\n[zone."co.uk"]\n', encoding="utf-8")  # me/org/net.uk absent
    monkeypatch.setattr(forbidden, "RULES", rules)
    monkeypatch.setattr(sys, "argv", ["forbidden.py"])
    assert forbidden.main() == 1
    # the file is untouched on refusal
    assert "forbidden_labels" not in rules.read_text()
