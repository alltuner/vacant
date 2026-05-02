// ABOUTME: Pure-Rust core library for vacant.
// ABOUTME: Holds the rules engine; DNS, RDAP, and disk-cache will land in their own modules.

pub mod disk_cache;
mod dns;
mod orchestrator;
pub mod rdap;
mod rules;

pub use disk_cache::{CacheError, CachedRow, DiskCache};
pub use dns::{DnsClient, DnsError, DnsVerdict, FullCheckJob, FullVerdict};
pub use orchestrator::{check_many, CheckResult};
pub use rules::{
    LoadError, MatchResult, PreCheck, RuleSet, RuleViolation, Status, ZoneMatch, ZonePolicy,
};

/// Smoke-test entry point so the maturin binding has something to call.
pub fn hello() -> &'static str {
    "vacant-core: ready"
}
