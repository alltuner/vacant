// ABOUTME: Property tests over the rule layer. No network, no DNS - just RuleSet.precheck().
// ABOUTME: Catches drift between the per-zone predicates encoded in TOML and the engine that runs them.

use std::str::FromStr;

use proptest::prelude::*;
use vacant::{PreCheck, RuleSet, Status};

const FIXTURE: &str = r#"
[default]
min_length = 1
max_length = 63
charset = "ldh"
no_edge_hyphen = true
no_tagged_hyphen = true

[zone.cat]
min_length = 3

[zone.eu]
min_length = 3

[zone.com]

[zone.uk]

[zone."co.uk"]

[zone."gov.uk"]
"#;

fn ruleset() -> RuleSet {
    RuleSet::from_str(FIXTURE).expect("fixture is valid")
}

fn ldh_label(min_len: usize, max_len: usize) -> impl Strategy<Value = String> {
    proptest::collection::vec(
        proptest::prop_oneof![
            proptest::char::range('a', 'z'),
            proptest::char::range('0', '9'),
            Just('-'),
        ],
        min_len..=max_len,
    )
    .prop_map(|cs| cs.into_iter().collect())
}

proptest! {
    #[test]
    fn min_length_violations_on_cat_are_reserved(
        label in ldh_label(1, 2)
            .prop_filter("must not start or end with '-'", |s| !s.starts_with('-') && !s.ends_with('-'))
    ) {
        let rs = ruleset();
        let domain = format!("{label}.cat");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Reserved);
                prop_assert!(detail.contains("min-length"), "got {}", detail);
            }
            other => prop_assert!(false, "expected reserved verdict, got {other:?}"),
        }
    }

    #[test]
    fn min_length_violations_on_eu_are_reserved(
        label in ldh_label(1, 2)
            .prop_filter("must not start or end with '-'", |s| !s.starts_with('-') && !s.ends_with('-'))
    ) {
        let rs = ruleset();
        let domain = format!("{label}.eu");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Reserved);
                prop_assert!(detail.contains("min-length"), "got {}", detail);
            }
            other => prop_assert!(false, "expected reserved verdict, got {other:?}"),
        }
    }

    #[test]
    fn leading_hyphen_is_reserved(
        rest in ldh_label(0, 30)
            .prop_filter("rest must not end with '-'", |s| !s.ends_with('-'))
    ) {
        let rs = ruleset();
        let domain = format!("-{rest}.com");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Reserved);
                prop_assert!(detail.contains("no-edge-hyphen"), "got {}", detail);
            }
            other => prop_assert!(false, "expected reserved verdict, got {other:?}"),
        }
    }

    #[test]
    fn trailing_hyphen_is_reserved(
        rest in ldh_label(0, 30)
            .prop_filter("rest must not start with '-'", |s| !s.starts_with('-'))
    ) {
        let rs = ruleset();
        let domain = format!("{rest}-.com");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Reserved);
                prop_assert!(detail.contains("no-edge-hyphen"), "got {}", detail);
            }
            other => prop_assert!(false, "expected reserved verdict, got {other:?}"),
        }
    }

    #[test]
    fn non_ldh_chars_are_reserved(
        prefix in ldh_label(1, 8)
            .prop_filter("prefix must not start with '-'", |s| !s.starts_with('-')),
        // Excludes '.', whitespace, and uppercase since precheck normalises those.
        bad in proptest::prop_oneof![
            Just('_'), Just('!'), Just('@'), Just('#'), Just('$'), Just('%'),
            Just('+'), Just('='), Just('?'), Just(','),
        ],
        suffix in ldh_label(1, 8)
            .prop_filter("suffix must not end with '-'", |s| !s.ends_with('-'))
    ) {
        let rs = ruleset();
        let label = format!("{prefix}{bad}{suffix}");
        let domain = format!("{label}.com");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Reserved);
                prop_assert!(detail.contains("charset-ldh"), "expected charset-ldh violation, got {}", detail);
            }
            other => prop_assert!(false, "expected reserved verdict, got {other:?}"),
        }
    }

    #[test]
    fn registered_zone_input_is_invalid(zone in proptest::prop_oneof![
        Just("com"), Just("uk"), Just("eu"), Just("cat"),
        Just("co.uk"), Just("gov.uk"),
    ]) {
        let rs = ruleset();
        match rs.precheck(zone) {
            PreCheck::Verdict { status, detail, .. } => {
                // Bare TLDs like "com"/"uk" don't contain a dot - those return InvalidNoTLD,
                // multi-label zones get the registry-suffix message.
                if zone.contains('.') {
                    prop_assert_eq!(status, Status::Invalid);
                    prop_assert!(detail.contains("registry suffix"), "got {}", detail);
                } else {
                    prop_assert_eq!(status, Status::Invalid);
                    prop_assert!(detail.contains("no TLD"), "got {}", detail);
                }
            }
            other => prop_assert!(false, "expected invalid verdict, got {other:?}"),
        }
    }

    #[test]
    fn extra_labels_above_zone_are_invalid(
        sub in ldh_label(1, 8)
            .prop_filter("must be valid LDH at edges", |s| !s.starts_with('-') && !s.ends_with('-')),
        label in ldh_label(3, 8)
            .prop_filter("must be valid LDH at edges", |s| !s.starts_with('-') && !s.ends_with('-')),
    ) {
        let rs = ruleset();
        let domain = format!("{sub}.{label}.cat");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Invalid);
                prop_assert!(detail.contains("below the registrable level"), "got {}", detail);
            }
            other => prop_assert!(false, "expected invalid verdict, got {other:?}"),
        }
    }

    #[test]
    fn unknown_tld_is_invalid(
        label in ldh_label(3, 8)
            .prop_filter("valid LDH", |s| !s.starts_with('-') && !s.ends_with('-')),
        bogus_tld in proptest::prop_oneof![
            Just("com3d"), Just("a"), Just("zz"), Just("notatld"), Just("xyzzy123"),
        ],
    ) {
        let rs = ruleset();
        let domain = format!("{label}.{bogus_tld}");
        match rs.precheck(&domain) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Invalid);
                prop_assert!(detail.contains("unknown TLD"), "got {}", detail);
            }
            other => prop_assert!(false, "expected invalid verdict, got {other:?} for {domain}"),
        }
    }

    #[test]
    fn empty_labels_are_invalid(
        bogus in proptest::prop_oneof![
            Just(".com"), Just(".adfa"), Just("..com"), Just("a..b.com"),
            Just("foo..com"), Just(".a.b"),
        ],
    ) {
        let rs = ruleset();
        match rs.precheck(bogus) {
            PreCheck::Verdict { status, detail, .. } => {
                prop_assert_eq!(status, Status::Invalid);
                prop_assert!(detail.contains("empty label"), "got {}", detail);
            }
            other => prop_assert!(false, "expected invalid verdict, got {other:?} for {bogus}"),
        }
    }

    #[test]
    fn well_formed_label_proceeds(
        label in ldh_label(3, 30)
            .prop_filter("must be valid LDH", |s| {
                !(s.starts_with('-')
                    || s.ends_with('-')
                    || s.len() >= 4 && &s[2..4] == "--" && !s.starts_with("xn--"))
            })
    ) {
        let rs = ruleset();
        let domain = format!("{label}.com");
        match rs.precheck(&domain) {
            PreCheck::Proceed { zone, label: emitted, registered, .. } => {
                prop_assert_eq!(zone, "com");
                prop_assert_eq!(&emitted, &label);
                prop_assert_eq!(registered, format!("{label}.com"));
            }
            other => prop_assert!(false, "expected proceed, got {other:?} for {domain}"),
        }
    }
}

const FORBIDDEN_FIXTURE: &str = r#"
[default]
min_length = 1
max_length = 63
charset = "ldh"

[zone."co.uk"]
forbidden_labels = ["gov", "police"]

[zone.com]
forbidden_labels = []
"#;

#[test]
fn forbidden_label_is_reserved_without_dns() {
    let rs = RuleSet::from_str(FORBIDDEN_FIXTURE).expect("fixture is valid");
    match rs.precheck("gov.co.uk") {
        PreCheck::Verdict {
            status,
            detail,
            zone,
            registered,
        } => {
            assert_eq!(status, Status::Reserved);
            assert!(detail.contains("forbidden-label"), "got {detail}");
            assert!(
                detail.contains("reserved by registry policy"),
                "got {detail}"
            );
            assert_eq!(zone, "co.uk");
            assert_eq!(registered, "gov.co.uk");
        }
        other => panic!("expected reserved verdict, got {other:?}"),
    }
}

#[test]
fn forbidden_label_match_is_case_insensitive() {
    let rs = RuleSet::from_str(FORBIDDEN_FIXTURE).expect("fixture is valid");
    match rs.precheck("POLICE.co.uk") {
        PreCheck::Verdict { status, .. } => assert_eq!(status, Status::Reserved),
        other => panic!("expected reserved verdict, got {other:?}"),
    }
}

#[test]
fn non_forbidden_label_in_guarded_zone_proceeds() {
    let rs = RuleSet::from_str(FORBIDDEN_FIXTURE).expect("fixture is valid");
    match rs.precheck("acme.co.uk") {
        PreCheck::Proceed { zone, label, .. } => {
            assert_eq!(zone, "co.uk");
            assert_eq!(label, "acme");
        }
        other => panic!("expected proceed, got {other:?}"),
    }
}

#[test]
fn empty_forbidden_labels_is_a_noop() {
    let rs = RuleSet::from_str(FORBIDDEN_FIXTURE).expect("fixture is valid");
    match rs.precheck("anything.com") {
        PreCheck::Proceed { zone, .. } => assert_eq!(zone, "com"),
        other => panic!("expected proceed, got {other:?}"),
    }
}
