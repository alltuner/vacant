// ABOUTME: End-to-end check pipeline: normalize, cache, precheck, batch DNS+RDAP, write back.
// ABOUTME: Single source of truth shared by the standalone binary and the PyO3 binding.

use crate::{
    normalize_input, DiskCache, DnsClient, FullCheckJob, FullVerdict, PreCheck, RuleSet, Status,
};

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub input: String,
    pub domain: String,
    pub zone: String,
    pub status: Status,
    pub detail: String,
    pub from_cache: bool,
}

pub fn check_many(
    rules: &RuleSet,
    dns: &DnsClient,
    cache: Option<&DiskCache>,
    inputs: &[String],
    cache_ttl_secs: i64,
    concurrency: usize,
    verify: bool,
) -> Vec<CheckResult> {
    let mut results: Vec<Option<CheckResult>> = vec![None; inputs.len()];
    let mut pending: Vec<(usize, String, String, Option<String>)> = Vec::new();

    for (i, raw) in inputs.iter().enumerate() {
        let Some(cleaned) = normalize_input(raw) else {
            results[i] = Some(CheckResult {
                input: raw.clone(),
                domain: String::new(),
                zone: String::new(),
                status: Status::Invalid,
                detail: "input is empty or not a recoverable host".to_string(),
                from_cache: false,
            });
            continue;
        };

        if !cleaned.contains('.') {
            results[i] = Some(CheckResult {
                input: raw.clone(),
                domain: cleaned,
                zone: String::new(),
                status: Status::Invalid,
                detail: "input has no TLD".to_string(),
                from_cache: false,
            });
            continue;
        }

        // Precheck is a pure in-memory verdict (Invalid/Reserved), so it is
        // authoritative and runs before the cache: a deterministic policy
        // verdict must never be masked by a stale network-derived cache row.
        match rules.precheck(&cleaned) {
            PreCheck::Verdict {
                status,
                detail,
                zone,
                registered,
            } => {
                results[i] = Some(CheckResult {
                    input: raw.clone(),
                    domain: registered,
                    zone,
                    status,
                    detail,
                    from_cache: false,
                });
            }
            PreCheck::Proceed {
                zone,
                registered,
                rdap,
                ..
            } => {
                if let Some(c) = cache {
                    if let Ok(Some(row)) = c.get(&registered, cache_ttl_secs) {
                        results[i] = Some(CheckResult {
                            input: raw.clone(),
                            domain: row.domain,
                            zone: row.zone,
                            status: parse_status(&row.status),
                            detail: row.detail,
                            from_cache: true,
                        });
                        continue;
                    }
                }
                pending.push((i, zone, registered, rdap));
            }
        }
    }

    if !pending.is_empty() {
        let jobs: Vec<FullCheckJob> = pending
            .iter()
            .map(|(_, zone, registered, rdap)| FullCheckJob {
                zone: zone.clone(),
                registered: registered.clone(),
                rdap_url: rdap.clone(),
            })
            .collect();
        let blocked = cache
            .and_then(|c| c.blocked_rdap_hosts().ok())
            .unwrap_or_default();
        let (verdicts, new_cooldowns) = dns.check_full_batch(jobs, concurrency, verify, blocked);
        if let Some(c) = cache {
            for (host, cooldown_secs) in new_cooldowns {
                let _ = c.block_rdap_host(&host, cooldown_secs);
            }
        }
        for ((i, zone, registered, _), v) in pending.into_iter().zip(verdicts) {
            results[i] = Some(verdict_to_result(&inputs[i], zone, registered, v));
        }
    }

    if let Some(c) = cache {
        for r in results.iter().flatten() {
            if r.from_cache
                || matches!(
                    r.status,
                    Status::Unknown | Status::Invalid | Status::Unconfirmed | Status::Reserved
                )
            {
                continue;
            }
            let _ = c.put(&r.domain, &r.zone, r.status.as_str(), &r.detail);
        }
    }

    results.into_iter().flatten().collect()
}

fn verdict_to_result(input: &str, zone: String, registered: String, v: FullVerdict) -> CheckResult {
    let status = match v.kind {
        "registered" => Status::Registered,
        "available" => Status::Available,
        "unconfirmed" => Status::Unconfirmed,
        _ => Status::Unknown,
    };
    CheckResult {
        input: input.to_string(),
        domain: registered,
        zone,
        status,
        detail: v.detail,
        from_cache: false,
    }
}

fn parse_status(s: &str) -> Status {
    match s {
        "available" => Status::Available,
        "registered" => Status::Registered,
        "reserved" => Status::Reserved,
        "invalid" => Status::Invalid,
        "unconfirmed" => Status::Unconfirmed,
        _ => Status::Unknown,
    }
}
