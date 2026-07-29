// ABOUTME: Guards the invariants the bundled rules.toml must hold to be usable at all.
// ABOUTME: Input is IDNA-encoded before lookup, so a zone stored as Unicode can never match.

use std::str::FromStr;

use vacant::{normalize_input, PreCheck, RuleSet};

const BUNDLED_RULES: &str = include_str!("../data/rules.toml");

fn rules() -> RuleSet {
    RuleSet::from_str(BUNDLED_RULES).expect("bundled rules parse")
}

fn precheck(rules: &RuleSet, input: &str) -> PreCheck {
    rules.precheck(&normalize_input(input).expect("input normalizes"))
}

#[test]
fn every_zone_name_is_ascii() {
    let rules = rules();
    let unicode: Vec<&str> = rules
        .zones
        .keys()
        .filter(|z| !z.is_ascii())
        .map(|z| z.as_str())
        .collect();
    assert!(
        unicode.is_empty(),
        "zones stored as Unicode are unreachable; run `just ingest-psl --force`: {unicode:?}"
    );
}

#[test]
fn idn_tld_resolves_to_its_zone() {
    match precheck(&rules(), "пример.москва") {
        PreCheck::Proceed { zone, rdap, .. } => {
            assert_eq!(zone, "xn--80adxhks");
            assert!(rdap.is_some(), ".москва should have an RDAP endpoint");
        }
        other => panic!("expected Proceed, got {other:?}"),
    }
}

#[test]
fn idn_second_level_suffix_is_a_zone_not_a_name() {
    let rules = rules();
    match precheck(&rules, "bodø.no") {
        PreCheck::Verdict { zone, .. } => assert_eq!(zone, "xn--bod-2na.no"),
        other => panic!("a public suffix is not registrable, got {other:?}"),
    }
    match precheck(&rules, "vacant-test.bodø.no") {
        PreCheck::Proceed { zone, label, .. } => {
            assert_eq!(zone, "xn--bod-2na.no");
            assert_eq!(label, "vacant-test");
        }
        other => panic!("expected Proceed, got {other:?}"),
    }
}
