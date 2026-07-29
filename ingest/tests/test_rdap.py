# ABOUTME: Tests for the RDAP bootstrap importer's pure decision logic.
# ABOUTME: Covers candidate generation for probing and which zones the bootstrap changes.
from __future__ import annotations

import rdap


def test_candidates_start_with_the_conventional_templates():
    assert rdap.candidates("fr", None) == [
        "https://rdap.nic.fr",
        "https://rdap.fr",
    ]


def test_whois_host_adds_the_registry_branded_candidate():
    # How .de's real endpoint is found: neither template resolves for it.
    assert rdap.candidates("de", "whois.denic.de")[-1] == "https://rdap.denic.de"


def test_whois_host_without_the_whois_prefix_is_used_whole():
    assert rdap.candidates("example", "registry.example")[-1] == "https://rdap.registry.example"


def test_whois_host_matching_a_template_is_not_repeated():
    assert rdap.candidates("fr", "whois.nic.fr") == rdap.candidates("fr", None)


def test_missing_tlds_skips_multi_level_suffixes_and_known_endpoints():
    zones = {
        "de": {},
        "uk": {},
        "co.uk": {},
        "com": {},
        "xn--p1ai": {},
        "cat": {"rdap": "https://rdap.nic.cat"},
    }
    assert rdap.missing_tlds(zones, {"com": "https://rdap.verisign.com/com/v1"}) == [
        "de",
        "uk",
        "xn--p1ai",
    ]


def test_compute_changes_only_reports_real_moves():
    zones = {
        "com": {"rdap": "https://rdap.verisign.com/com/v1"},
        "de": {},
        "co.uk": {"rdap": "https://stale.example"},
    }
    mapping = {
        "com": "https://rdap.verisign.com/com/v1",
        "de": "https://rdap.denic.de",
        "uk": "https://rdap.nominet.uk/uk",
    }
    assert rdap.compute_changes(zones, mapping) == [
        ("de", None, "https://rdap.denic.de"),
        ("co.uk", "https://stale.example", "https://rdap.nominet.uk/uk"),
    ]
