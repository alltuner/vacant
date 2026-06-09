from vacant.mcp import check_domains


def test_check_domains_returns_status_per_input():
    out = check_domains(["google.com", "this-is-clearly-not-a-domain.example"])
    assert [r.domain for r in out] == ["google.com", "this-is-clearly-not-a-domain.example"]
    assert out[0].status in {"registered", "reserved"}
    assert out[1].status == "invalid"


def test_check_domains_skips_blank_and_comment_lines():
    out = check_domains(["", "  ", "# a comment", "google.com"])
    assert [r.domain for r in out] == ["google.com"]
