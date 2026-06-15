// ABOUTME: TOML-driven rules engine: zone matching plus per-zone label predicates.
// ABOUTME: Ports the Python rules.py / checker pre-DNS path so the Rust engine can A/B against it.

use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Available,
    Registered,
    Reserved,
    Invalid,
    Unconfirmed,
    Unknown,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Available => "available",
            Status::Registered => "registered",
            Status::Reserved => "reserved",
            Status::Invalid => "invalid",
            Status::Unconfirmed => "unconfirmed",
            Status::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleViolation {
    pub rule: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneMatch {
    pub zone: String,
    pub label: String,
    pub registered: String,
    pub extra: Vec<String>,
}

pub enum MatchResult<'a> {
    Found {
        zone_match: ZoneMatch,
        policy: &'a ZonePolicy,
    },
    Fallback {
        zone: String,
        label: String,
        registered: String,
        policy: &'a ZonePolicy,
    },
}

/// What the rule layer can determine before any DNS work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreCheck {
    /// The label and zone resolved cleanly; the caller should now do DNS lookups.
    Proceed {
        zone: String,
        label: String,
        registered: String,
        rdap: Option<String>,
    },
    /// Short-circuit answer; caller skips DNS.
    Verdict {
        status: Status,
        detail: String,
        zone: String,
        registered: String,
    },
}

#[derive(Debug)]
struct Predicate {
    name: &'static str,
    message: String,
    test: PredicateFn,
}

#[derive(Debug)]
enum PredicateFn {
    MinLength(usize),
    MaxLength(usize),
    Ldh,
    NoEdgeHyphen,
    NoTaggedHyphen,
    Pattern(Regex),
    ForbidPattern(Regex),
    ForbiddenLabel(HashSet<String>),
}

impl Predicate {
    fn check(&self, label: &str) -> bool {
        match &self.test {
            PredicateFn::MinLength(n) => label.chars().count() >= *n,
            PredicateFn::MaxLength(n) => label.chars().count() <= *n,
            PredicateFn::Ldh => {
                !label.is_empty()
                    && label
                        .chars()
                        .all(|c| c.is_ascii() && (c.is_ascii_alphanumeric() || c == '-'))
            }
            PredicateFn::NoEdgeHyphen => !(label.starts_with('-') || label.ends_with('-')),
            PredicateFn::NoTaggedHyphen => {
                if label.starts_with("xn--") {
                    return true;
                }
                let bytes = label.as_bytes();
                !(bytes.len() >= 4 && bytes[2] == b'-' && bytes[3] == b'-')
            }
            PredicateFn::Pattern(re) => re.is_match(label) && full_match(re, label),
            PredicateFn::ForbidPattern(re) => !re.is_match(label),
            PredicateFn::ForbiddenLabel(set) => !set.contains(label),
        }
    }
}

fn full_match(re: &Regex, label: &str) -> bool {
    match re.find(label) {
        Some(m) => m.start() == 0 && m.end() == label.len(),
        None => false,
    }
}

#[derive(Debug)]
pub struct ZonePolicy {
    pub zone: String,
    predicates: Vec<Predicate>,
    pub rdap: Option<String>,
}

impl ZonePolicy {
    pub fn evaluate(&self, label: &str) -> Option<RuleViolation> {
        for p in &self.predicates {
            if !p.check(label) {
                return Some(RuleViolation {
                    rule: p.name.to_string(),
                    message: p.message.clone(),
                });
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct RuleSet {
    pub default: ZonePolicy,
    pub zones: HashMap<String, ZonePolicy>,
}

impl std::str::FromStr for RuleSet {
    type Err = LoadError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let raw: RawDocument = toml::from_str(text)?;
        let default_spec = raw.default.unwrap_or_default();
        let default = build_policy("*", &default_spec, &default_spec)?;

        let mut zones = HashMap::new();
        if let Some(table) = raw.zone {
            for (name, spec) in table {
                let merged = merge(&default_spec, &spec);
                let lower = name.to_ascii_lowercase();
                let policy = build_policy(&lower, &merged, &default_spec)?;
                zones.insert(lower, policy);
            }
        }
        Ok(RuleSet { default, zones })
    }
}

impl RuleSet {
    pub fn policy_for(&self, zone: &str) -> &ZonePolicy {
        self.zones
            .get(&zone.to_ascii_lowercase())
            .unwrap_or(&self.default)
    }

    pub fn match_zone(&self, domain: &str) -> Option<ZoneMatch> {
        let cleaned = normalize(domain);
        if cleaned.is_empty() || self.zones.contains_key(&cleaned) {
            return None;
        }
        let labels: Vec<&str> = cleaned.split('.').collect();
        for j in 1..labels.len() {
            let candidate = labels[j..].join(".");
            if self.zones.contains_key(&candidate) {
                let label = labels[j - 1].to_string();
                let registered = labels[j - 1..].join(".");
                let extra = labels[..j - 1].iter().map(|s| s.to_string()).collect();
                return Some(ZoneMatch {
                    zone: candidate,
                    label,
                    registered,
                    extra,
                });
            }
        }
        None
    }

    /// Run the full pre-DNS pipeline (the Python checker.check() prologue, end-to-end).
    pub fn precheck(&self, raw_domain: &str) -> PreCheck {
        let cleaned = normalize(raw_domain);
        if !cleaned.contains('.') {
            return PreCheck::Verdict {
                status: Status::Invalid,
                detail: "input has no TLD".to_string(),
                zone: String::new(),
                registered: cleaned,
            };
        }
        if cleaned.split('.').any(|l| l.is_empty()) {
            return PreCheck::Verdict {
                status: Status::Invalid,
                detail: "malformed: empty label (leading/trailing dot or '..')".to_string(),
                zone: String::new(),
                registered: cleaned,
            };
        }
        if self.zones.contains_key(&cleaned) {
            return PreCheck::Verdict {
                status: Status::Invalid,
                detail: format!("'{cleaned}' is a registry suffix, not a registrable name"),
                zone: cleaned.clone(),
                registered: cleaned,
            };
        }

        let m = match self.match_zone(&cleaned) {
            Some(m) => m,
            None => {
                let tld = cleaned.rsplit('.').next().unwrap_or("");
                return PreCheck::Verdict {
                    status: Status::Invalid,
                    detail: format!("unknown TLD '.{tld}' (not in the public suffix list)"),
                    zone: tld.to_string(),
                    registered: cleaned,
                };
            }
        };
        if !m.extra.is_empty() {
            let detail = format!(
                "'{cleaned}' is below the registrable level for zone '{}' (registrable name would be '{}')",
                m.zone, m.registered
            );
            return PreCheck::Verdict {
                status: Status::Invalid,
                detail,
                zone: m.zone,
                registered: m.registered,
            };
        }
        let policy = self.policy_for(&m.zone);
        let (zone, label, registered) = (m.zone, m.label, m.registered);

        if let Some(violation) = policy.evaluate(&label) {
            return PreCheck::Verdict {
                status: Status::Reserved,
                detail: format!("{}: {}", violation.rule, violation.message),
                zone,
                registered,
            };
        }

        PreCheck::Proceed {
            zone,
            label,
            registered,
            rdap: policy.rdap.clone(),
        }
    }
}

fn normalize(domain: &str) -> String {
    domain.trim().trim_end_matches('.').to_ascii_lowercase()
}

#[derive(Debug, Default, Deserialize)]
struct RawDocument {
    #[serde(default)]
    default: Option<RawSpec>,
    #[serde(default)]
    zone: Option<HashMap<String, RawSpec>>,
}

#[derive(Debug, Default, Clone, Deserialize)]
struct RawSpec {
    #[serde(default)]
    min_length: Option<i64>,
    #[serde(default)]
    max_length: Option<i64>,
    #[serde(default)]
    charset: Option<String>,
    #[serde(default)]
    no_edge_hyphen: Option<bool>,
    #[serde(default)]
    no_tagged_hyphen: Option<bool>,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    forbid_pattern: Option<String>,
    #[serde(default)]
    forbidden_labels: Option<Vec<String>>,
    #[serde(default)]
    rdap: Option<String>,
}

fn merge(default: &RawSpec, override_: &RawSpec) -> RawSpec {
    RawSpec {
        min_length: override_.min_length.or(default.min_length),
        max_length: override_.max_length.or(default.max_length),
        charset: override_
            .charset
            .clone()
            .or_else(|| default.charset.clone()),
        no_edge_hyphen: override_.no_edge_hyphen.or(default.no_edge_hyphen),
        no_tagged_hyphen: override_.no_tagged_hyphen.or(default.no_tagged_hyphen),
        pattern: override_
            .pattern
            .clone()
            .or_else(|| default.pattern.clone()),
        forbid_pattern: override_
            .forbid_pattern
            .clone()
            .or_else(|| default.forbid_pattern.clone()),
        forbidden_labels: override_
            .forbidden_labels
            .clone()
            .or_else(|| default.forbidden_labels.clone()),
        rdap: override_.rdap.clone().or_else(|| default.rdap.clone()),
    }
}

fn build_policy(zone: &str, spec: &RawSpec, _root: &RawSpec) -> Result<ZonePolicy, LoadError> {
    let mut predicates = Vec::new();
    if let Some(n) = spec.min_length {
        let n = usize::try_from(n).map_err(|_| LoadError::BadValue("min_length"))?;
        predicates.push(Predicate {
            name: "min-length",
            message: format!("label must be at least {n} characters"),
            test: PredicateFn::MinLength(n),
        });
    }
    if let Some(n) = spec.max_length {
        let n = usize::try_from(n).map_err(|_| LoadError::BadValue("max_length"))?;
        predicates.push(Predicate {
            name: "max-length",
            message: format!("label must be at most {n} characters"),
            test: PredicateFn::MaxLength(n),
        });
    }
    if let Some(charset) = &spec.charset {
        match charset.as_str() {
            "ldh" => predicates.push(Predicate {
                name: "charset-ldh",
                message: "label must contain only letters, digits, and hyphens".to_string(),
                test: PredicateFn::Ldh,
            }),
            other => return Err(LoadError::UnknownCharset(other.to_string())),
        }
    }
    if spec.no_edge_hyphen.unwrap_or(false) {
        predicates.push(Predicate {
            name: "no-edge-hyphen",
            message: "label cannot start or end with '-'".to_string(),
            test: PredicateFn::NoEdgeHyphen,
        });
    }
    if spec.no_tagged_hyphen.unwrap_or(false) {
        predicates.push(Predicate {
            name: "no-tagged-hyphen",
            message: "label cannot have '-' in positions 3 and 4".to_string(),
            test: PredicateFn::NoTaggedHyphen,
        });
    }
    if let Some(pat) = &spec.pattern {
        let re = Regex::new(pat).map_err(LoadError::Regex)?;
        predicates.push(Predicate {
            name: "pattern",
            message: format!("label must match {pat}"),
            test: PredicateFn::Pattern(re),
        });
    }
    if let Some(pat) = &spec.forbid_pattern {
        let re = Regex::new(pat).map_err(LoadError::Regex)?;
        predicates.push(Predicate {
            name: "forbid-pattern",
            message: format!("label must not match {pat}"),
            test: PredicateFn::ForbidPattern(re),
        });
    }
    if let Some(labels) = &spec.forbidden_labels {
        let set: HashSet<String> = labels.iter().map(|l| l.to_ascii_lowercase()).collect();
        if !set.is_empty() {
            predicates.push(Predicate {
                name: "forbidden-label",
                message: "reserved by registry policy".to_string(),
                test: PredicateFn::ForbiddenLabel(set),
            });
        }
    }
    Ok(ZonePolicy {
        zone: zone.to_string(),
        predicates,
        rdap: spec.rdap.clone(),
    })
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("toml parse: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("regex: {0}")]
    Regex(regex::Error),
    #[error("unknown charset: {0}")]
    UnknownCharset(String),
    #[error("invalid value for {0}")]
    BadValue(&'static str),
}
