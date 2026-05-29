// ABOUTME: End-to-end tests for the orchestrator's cache-write contract.
// ABOUTME: Guards against re-introducing input-shape Invalid results poisoning the cache.

use std::str::FromStr;
use std::time::Duration;

use tempfile::TempDir;
use vacant::{check_many, DiskCache, DnsClient, RuleSet};

const FIXTURE: &str = r#"
[default]
min_length = 1
max_length = 63
charset = "ldh"
no_edge_hyphen = true
no_tagged_hyphen = true

[zone.com]
"#;

fn rules() -> RuleSet {
    RuleSet::from_str(FIXTURE).expect("fixture is valid")
}

fn dns() -> DnsClient {
    DnsClient::new(Duration::from_secs(1)).expect("dns client init")
}

#[test]
fn invalid_below_registrable_does_not_poison_cache() {
    let tmp = TempDir::new().expect("tempdir");
    let cache = DiskCache::open(&tmp.path().join("results.db")).expect("open cache");
    let rs = rules();
    let dc = dns();

    // 1) Seed the cache by checking a subdomain — precheck returns Invalid
    //    ("below the registrable level"). No DNS is needed for the verdict.
    let inputs = vec!["vacant.alltuner.com".to_string()];
    let results = check_many(&rs, &dc, Some(&cache), &inputs, 86_400, 1, false);
    assert_eq!(results[0].status.as_str(), "invalid");
    assert_eq!(results[0].domain, "alltuner.com");

    // 2) The cache must NOT contain a row for the registrable name —
    //    otherwise any later query for alltuner.com (or its siblings/parents)
    //    would surface this stale "below the registrable level" detail.
    let row = cache.get("alltuner.com", 86_400).expect("cache get");
    assert!(
        row.is_none(),
        "cache was poisoned with an Invalid result keyed by registrable name: {row:?}"
    );

    // And nothing under the input-shape key either.
    let row = cache.get("vacant.alltuner.com", 86_400).expect("cache get");
    assert!(row.is_none());
}

#[test]
fn rdap_cooldown_round_trips_and_expires() {
    let tmp = TempDir::new().expect("tempdir");
    let cache = DiskCache::open(&tmp.path().join("results.db")).expect("open cache");

    cache
        .block_rdap_host("rdap.example.test", 300)
        .expect("block host");
    // An already-elapsed cooldown must not come back as blocked.
    cache
        .block_rdap_host("rdap.expired.test", -10)
        .expect("block host");

    let blocked = cache.blocked_rdap_hosts().expect("read blocked");
    assert!(blocked.contains("rdap.example.test"));
    assert!(!blocked.contains("rdap.expired.test"));
}
