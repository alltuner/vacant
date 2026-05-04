from vacant import DiskCache, Result, Status, check, check_many


def test_check_many_preserves_input_order_and_classifies():
    inputs = ["google.com", "this-is-clearly-not-a-domain.example", ""]
    results = check_many(inputs)
    assert [r.input for r in results] == inputs
    assert results[0].status in {Status.REGISTERED, Status.RESERVED}
    assert results[1].status is Status.INVALID
    assert results[2].status is Status.INVALID


def test_check_single_invalid():
    r = check("nope")
    assert r.status is Status.INVALID


def test_invalid_below_registrable_does_not_poison_cache(tmp_path):
    cache = DiskCache(tmp_path / "results.db")
    results = check_many(["vacant.alltuner.com"], cache=cache)
    assert results[0].status is Status.INVALID
    assert results[0].domain == "alltuner.com"
    assert cache.get("alltuner.com", ttl=86_400) is None
    assert cache.get("vacant.alltuner.com", ttl=86_400) is None


def test_disk_cache_put_skips_invalid(tmp_path):
    cache = DiskCache(tmp_path / "results.db")
    cache.put(
        Result(
            input="vacant.alltuner.com",
            domain="alltuner.com",
            zone="com",
            status=Status.INVALID,
            detail="below the registrable level",
        )
    )
    assert cache.get("alltuner.com", ttl=86_400) is None
