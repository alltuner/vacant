// ABOUTME: Library surface for vacant: rules engine, DNS, RDAP, and disk-cache.
// ABOUTME: The vacant binary in src/main.rs is the primary consumer.

pub mod disk_cache;
mod dns;
mod normalize;
mod orchestrator;
pub mod rdap;
mod rules;

pub use disk_cache::{CacheError, CachedRow, DiskCache};
pub use dns::{DnsClient, DnsError, DnsVerdict, FullCheckJob, FullVerdict};
pub use normalize::normalize_input;
pub use orchestrator::{check_many, CheckResult};
pub use rules::{
    LoadError, MatchResult, PreCheck, RuleSet, RuleViolation, Status, ZoneMatch, ZonePolicy,
};
