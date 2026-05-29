// ABOUTME: Async DNS path: parent-zone NS via hickory's async resolver, AA query via tokio UDP.
// ABOUTME: Sync entry points block on the same code; batch entry runs N queries concurrently.

use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use hickory_proto::op::{Message, MessageType, OpCode, Query, ResponseCode};
use hickory_proto::rr::{Name, RData, RecordType};
use hickory_proto::ProtoError;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::net::runtime::TokioRuntimeProvider;
use hickory_resolver::TokioResolver;
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use tokio::sync::Semaphore;
use tokio::time::timeout as tokio_timeout;

use crate::rdap::{self, RdapOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsVerdict {
    Registered { detail: String },
    Available { detail: String },
    Nodata { detail: String },
    Failure { detail: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullVerdict {
    pub kind: &'static str, // registered | available | failure
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct FullCheckJob {
    pub zone: String,
    pub registered: String,
    pub rdap_url: Option<String>,
}

#[derive(Debug, Error)]
pub enum DnsError {
    #[error("resolver init: {0}")]
    Init(String),
    #[error("name parse: {0}")]
    Name(#[from] ProtoError),
}

/// Per-host RDAP rate: one request in flight per host with a deliberate gap
/// between dispatches, landing near the ~1-2 req/s that registries like
/// Identity Digital tolerate before returning 429.
const RDAP_PER_HOST_CONCURRENCY: usize = 1;
const RDAP_PER_HOST_MIN_GAP: Duration = Duration::from_millis(500);

/// How long to leave a 429'd RDAP host alone on later runs. We honour the
/// server's `Retry-After` when it sent one, falling back to a short default,
/// and cap it so a multi-hour block doesn't lock the host out for a whole day.
const RDAP_COOLDOWN_DEFAULT: Duration = Duration::from_secs(300);
const RDAP_COOLDOWN_CAP: Duration = Duration::from_secs(3600);

/// How long an unresponsive nameserver stays in the penalty box.
/// Matches Python NameserverCache.host_cooldown.
const NS_HOST_COOLDOWN: Duration = Duration::from_secs(300);

#[derive(Default)]
struct HostHealth {
    blocked: Mutex<HashMap<String, (Instant, String)>>,
}

impl HostHealth {
    fn is_healthy(&self, host: &str) -> bool {
        let mut guard = self.blocked.lock().expect("host health lock");
        match guard.get(host) {
            Some((until, _)) if Instant::now() < *until => false,
            Some(_) => {
                guard.remove(host);
                true
            }
            None => true,
        }
    }

    fn mark(&self, host: &str, reason: String) {
        let mut guard = self.blocked.lock().expect("host health lock");
        guard.insert(
            host.to_string(),
            (Instant::now() + NS_HOST_COOLDOWN, reason),
        );
    }
}

#[derive(Default)]
struct RdapThrottle {
    hosts: Mutex<HashMap<String, Arc<Semaphore>>>,
}

impl RdapThrottle {
    fn permit_for(&self, host: &str) -> Arc<Semaphore> {
        let mut guard = self.hosts.lock().expect("rdap throttle lock");
        guard
            .entry(host.to_string())
            .or_insert_with(|| Arc::new(Semaphore::new(RDAP_PER_HOST_CONCURRENCY)))
            .clone()
    }
}

pub struct DnsClient {
    resolver: TokioResolver,
    runtime: Arc<Runtime>,
    http: reqwest::Client,
    rdap_throttle: Arc<RdapThrottle>,
    host_health: Arc<HostHealth>,
    timeout: Duration,
}

impl DnsClient {
    pub fn new(timeout: Duration) -> Result<Self, DnsError> {
        let runtime = Runtime::new().map_err(|e| DnsError::Init(e.to_string()))?;
        let runtime = Arc::new(runtime);
        let resolver = runtime
            .block_on(async {
                let builder = TokioResolver::builder_tokio().unwrap_or_else(|_| {
                    TokioResolver::builder_with_config(
                        ResolverConfig::default(),
                        TokioRuntimeProvider::default(),
                    )
                });
                builder.build()
            })
            .map_err(|e| DnsError::Init(e.to_string()))?;
        let http = rdap::build_client(timeout).map_err(|e| DnsError::Init(e.to_string()))?;
        Ok(Self {
            resolver,
            runtime,
            http,
            rdap_throttle: Arc::new(RdapThrottle::default()),
            host_health: Arc::new(HostHealth::default()),
            timeout,
        })
    }

    pub fn check_authoritative(&self, zone: &str, registered: &str) -> DnsVerdict {
        self.runtime.block_on(check_authoritative_async(
            &self.resolver,
            &self.host_health,
            zone,
            registered,
            self.timeout,
        ))
    }

    /// Run a batch of (zone, registered) pairs concurrently. Order preserved.
    pub fn check_batch(&self, pairs: Vec<(String, String)>, concurrency: usize) -> Vec<DnsVerdict> {
        let resolver = self.resolver.clone();
        let health = self.host_health.clone();
        let timeout = self.timeout;
        self.runtime.block_on(async move {
            let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
            let tasks: Vec<_> = pairs
                .into_iter()
                .map(|(zone, registered)| {
                    let sem = semaphore.clone();
                    let res = resolver.clone();
                    let h = health.clone();
                    tokio::spawn(async move {
                        let _permit = sem.acquire_owned().await.expect("semaphore not closed");
                        check_authoritative_async(&res, &h, &zone, &registered, timeout).await
                    })
                })
                .collect();
            let mut out = Vec::with_capacity(tasks.len());
            for t in tasks {
                out.push(t.await.unwrap_or_else(|e| DnsVerdict::Failure {
                    detail: format!("task join: {e}"),
                }));
            }
            out
        })
    }

    /// Run DNS + RDAP-on-NODATA concurrently for each job. Order preserved.
    ///
    /// `blocked_hosts` are RDAP hosts in a persisted cooldown: their probes are
    /// skipped so a rate-limited registry isn't re-hammered across runs. Returns
    /// the verdicts plus the hosts that rate-limited us this run, mapped to how
    /// long (seconds) they should be left alone, for the caller to persist.
    pub fn check_full_batch(
        &self,
        jobs: Vec<FullCheckJob>,
        concurrency: usize,
        verify: bool,
        blocked_hosts: HashSet<String>,
    ) -> (Vec<FullVerdict>, HashMap<String, i64>) {
        let resolver = self.resolver.clone();
        let http = self.http.clone();
        let throttle = self.rdap_throttle.clone();
        let health = self.host_health.clone();
        let timeout = self.timeout;
        self.runtime.block_on(async move {
            let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
            let skip = Arc::new(Mutex::new(blocked_hosts));
            let cooldowns = Arc::new(Mutex::new(HashMap::new()));
            let tasks: Vec<_> = jobs
                .into_iter()
                .map(|job| {
                    let sem = semaphore.clone();
                    let res = resolver.clone();
                    let http = http.clone();
                    let throttle = throttle.clone();
                    let h = health.clone();
                    let skip = skip.clone();
                    let cooldowns = cooldowns.clone();
                    tokio::spawn(async move {
                        let _permit = sem.acquire_owned().await.expect("semaphore not closed");
                        run_full_job(
                            &res, &http, &throttle, &h, &skip, &cooldowns, job, timeout, verify,
                        )
                        .await
                    })
                })
                .collect();
            let mut out = Vec::with_capacity(tasks.len());
            for t in tasks {
                out.push(t.await.unwrap_or_else(|e| FullVerdict {
                    kind: "failure",
                    detail: format!("task join: {e}"),
                }));
            }
            let cooldowns = cooldowns.lock().expect("rdap cooldown lock").clone();
            (out, cooldowns)
        })
    }
}

/// What to do with a DNS verdict before any RDAP network call.
///
/// A delegation is a hard "registered". A DNS failure is a hard "failure". But
/// "not in the zone" (NXDOMAIN) and "in the zone, no NS" (NODATA) only tell us a
/// name isn't *delegated* — not whether it's *registrable*. Held, suspended, and
/// pending-delete domains are registered yet answer NXDOMAIN, so we never call
/// those "available" on DNS alone. Without `--verify` they stay `unconfirmed`;
/// with `--verify` and an RDAP endpoint we confirm against the registry.
#[derive(Debug, PartialEq, Eq)]
enum Decision {
    Report(&'static str),
    Confirm,
}

fn decide(verdict: &DnsVerdict, verify: bool, has_rdap: bool) -> Decision {
    match verdict {
        DnsVerdict::Registered { .. } => Decision::Report("registered"),
        DnsVerdict::Failure { .. } => Decision::Report("failure"),
        DnsVerdict::Available { .. } | DnsVerdict::Nodata { .. } => {
            if verify && has_rdap {
                Decision::Confirm
            } else {
                Decision::Report("unconfirmed")
            }
        }
    }
}

/// Fold an RDAP probe result into a final verdict. Only the registry can promote
/// an undelegated name to `available` (404) or `registered` (200); an inconclusive
/// probe leaves it `unconfirmed`.
fn combine_rdap(base_detail: &str, outcome: RdapOutcome) -> FullVerdict {
    match outcome {
        RdapOutcome::Registered { detail } => FullVerdict {
            kind: "registered",
            detail: format!("{base_detail}; {detail}"),
        },
        RdapOutcome::Available { detail } => FullVerdict {
            kind: "available",
            detail: format!("{base_detail}; {detail}"),
        },
        RdapOutcome::RateLimited { detail, .. } | RdapOutcome::Inconclusive { detail } => {
            FullVerdict {
                kind: "unconfirmed",
                detail: format!("{base_detail}; {detail}"),
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_full_job(
    resolver: &TokioResolver,
    http: &reqwest::Client,
    throttle: &RdapThrottle,
    health: &HostHealth,
    skip: &Mutex<HashSet<String>>,
    cooldowns: &Mutex<HashMap<String, i64>>,
    job: FullCheckJob,
    timeout: Duration,
    verify: bool,
) -> FullVerdict {
    let dns =
        check_authoritative_async(resolver, health, &job.zone, &job.registered, timeout).await;
    let detail = match &dns {
        DnsVerdict::Registered { detail }
        | DnsVerdict::Available { detail }
        | DnsVerdict::Failure { detail }
        | DnsVerdict::Nodata { detail } => detail.clone(),
    };
    match decide(&dns, verify, job.rdap_url.is_some()) {
        Decision::Report("registered") => FullVerdict {
            kind: "registered",
            detail,
        },
        Decision::Report("failure") => FullVerdict {
            kind: "failure",
            detail,
        },
        Decision::Report(_) => {
            // Undelegated and not confirmed: probably free, but unverified.
            let detail = if verify {
                format!("{detail} (no RDAP endpoint for zone '{}')", job.zone)
            } else {
                detail
            };
            FullVerdict {
                kind: "unconfirmed",
                detail,
            }
        }
        Decision::Confirm => {
            let base = job.rdap_url.expect("confirm implies an rdap endpoint");
            let host = rdap_host(&base).unwrap_or_else(|| base.clone());
            let semaphore = throttle.permit_for(&host);
            let permit = semaphore.acquire_owned().await.expect("rdap permit");
            // Re-check under the per-host permit: a host that rate-limited an
            // earlier task this run (or is in a persisted cooldown) is skipped
            // rather than probed again.
            if skip.lock().expect("rdap skip lock").contains(&host) {
                drop(permit);
                return FullVerdict {
                    kind: "unconfirmed",
                    detail: format!("{detail}; RDAP skipped: {host} in cooldown"),
                };
            }
            let outcome = rdap::lookup(http, &job.registered, &base).await;
            // Hold the permit for a short cool-down so back-to-back tasks don't
            // race the same host. drop(permit) happens after the sleep.
            tokio::time::sleep(RDAP_PER_HOST_MIN_GAP).await;
            drop(permit);
            if let RdapOutcome::RateLimited { retry_after, .. } = &outcome {
                let secs = retry_after
                    .unwrap_or(RDAP_COOLDOWN_DEFAULT)
                    .min(RDAP_COOLDOWN_CAP)
                    .as_secs() as i64;
                skip.lock().expect("rdap skip lock").insert(host.clone());
                cooldowns
                    .lock()
                    .expect("rdap cooldown lock")
                    .insert(host, secs);
            }
            combine_rdap(&detail, outcome)
        }
    }
}

fn rdap_host(base_url: &str) -> Option<String> {
    let stripped = base_url.split("://").nth(1).unwrap_or(base_url);
    stripped.split('/').next().map(|s| s.to_ascii_lowercase())
}

async fn check_authoritative_async(
    resolver: &TokioResolver,
    health: &HostHealth,
    zone: &str,
    registered: &str,
    timeout: Duration,
) -> DnsVerdict {
    let zone_name = match Name::from_str(zone) {
        Ok(n) => n,
        Err(e) => {
            return DnsVerdict::Failure {
                detail: format!("bad zone {zone}: {e}"),
            }
        }
    };
    let all_nameservers: Vec<String> = match resolver.lookup(zone_name, RecordType::NS).await {
        Ok(lookup) => lookup
            .answers()
            .iter()
            .filter_map(|record| match &record.data {
                RData::NS(ns) => Some(ns.0.to_utf8().trim_end_matches('.').to_string()),
                _ => None,
            })
            .collect(),
        Err(e) => {
            return DnsVerdict::Failure {
                detail: format!("zone NS lookup failed: {e}"),
            }
        }
    };
    if all_nameservers.is_empty() {
        return DnsVerdict::Failure {
            detail: format!("zone {zone} has no NS"),
        };
    }

    let healthy: Vec<String> = all_nameservers
        .into_iter()
        .filter(|ns| health.is_healthy(ns))
        .collect();
    if healthy.is_empty() {
        return DnsVerdict::Failure {
            detail: format!("all {zone} nameservers in cooldown"),
        };
    }

    let registered_name = match Name::from_str(registered) {
        Ok(n) => n,
        Err(e) => {
            return DnsVerdict::Failure {
                detail: format!("bad domain {registered}: {e}"),
            }
        }
    };

    let mut last_error: Option<String> = None;
    for ns in healthy {
        let ip = match resolver.lookup_ip(&ns).await {
            Ok(addrs) => match addrs
                .iter()
                .find(|ip| ip.is_ipv4())
                .or_else(|| addrs.iter().next())
            {
                Some(ip) => ip,
                None => {
                    health.mark(&ns, format!("no A/AAAA for {ns}"));
                    last_error = Some(format!("no A/AAAA for {ns}"));
                    continue;
                }
            },
            Err(e) => {
                health.mark(&ns, format!("resolve: {e}"));
                last_error = Some(format!("resolve {ns}: {e}"));
                continue;
            }
        };
        match query_authoritative_async(&registered_name, ip, timeout).await {
            Ok(response) => {
                if let Some(verdict) = classify(&response, &ns) {
                    return verdict;
                }
                last_error = Some(format!("unexpected rcode via {ns}"));
            }
            Err(e) => {
                health.mark(&ns, format!("query: {e}"));
                last_error = Some(format!("query {ns}: {e}"));
            }
        }
    }
    DnsVerdict::Failure {
        detail: last_error.unwrap_or_else(|| "all nameservers failed".to_string()),
    }
}

async fn query_authoritative_async(
    name: &Name,
    ip: IpAddr,
    timeout: Duration,
) -> Result<Message, String> {
    let mut msg = Message::new(rand::random::<u16>(), MessageType::Query, OpCode::Query);
    msg.metadata.recursion_desired = false;
    msg.add_query(Query::query(name.clone(), RecordType::NS));

    let bytes = msg.to_vec().map_err(|e| e.to_string())?;
    let bind_addr = if ip.is_ipv6() { "[::]:0" } else { "0.0.0.0:0" };
    let sock = UdpSocket::bind(bind_addr)
        .await
        .map_err(|e| e.to_string())?;
    sock.connect(SocketAddr::new(ip, 53))
        .await
        .map_err(|e| e.to_string())?;

    let send = sock.send(&bytes);
    tokio_timeout(timeout, send)
        .await
        .map_err(|_| "send timeout".to_string())?
        .map_err(|e| e.to_string())?;

    let mut buf = vec![0u8; 4096];
    let recv = sock.recv(&mut buf);
    let n = tokio_timeout(timeout, recv)
        .await
        .map_err(|_| "recv timeout".to_string())?
        .map_err(|e| e.to_string())?;
    Message::from_vec(&buf[..n]).map_err(|e| e.to_string())
}

fn classify(response: &Message, ns: &str) -> Option<DnsVerdict> {
    match response.metadata.response_code {
        ResponseCode::NXDomain => Some(DnsVerdict::Available {
            detail: format!("NXDOMAIN via {ns}"),
        }),
        ResponseCode::NoError => {
            let has_ns = response
                .answers
                .iter()
                .chain(response.authorities.iter())
                .any(|r| r.record_type() == RecordType::NS);
            if has_ns {
                Some(DnsVerdict::Registered {
                    detail: format!("delegation via {ns}"),
                })
            } else {
                Some(DnsVerdict::Nodata {
                    detail: format!("NODATA via {ns}"),
                })
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn available() -> DnsVerdict {
        DnsVerdict::Available {
            detail: "NXDOMAIN via ns1".to_string(),
        }
    }
    fn nodata() -> DnsVerdict {
        DnsVerdict::Nodata {
            detail: "NODATA via ns1".to_string(),
        }
    }
    fn registered() -> DnsVerdict {
        DnsVerdict::Registered {
            detail: "delegation via ns1".to_string(),
        }
    }
    fn failure() -> DnsVerdict {
        DnsVerdict::Failure {
            detail: "all nameservers failed".to_string(),
        }
    }

    #[test]
    fn delegation_is_always_registered() {
        for verify in [false, true] {
            for has_rdap in [false, true] {
                assert_eq!(
                    decide(&registered(), verify, has_rdap),
                    Decision::Report("registered")
                );
            }
        }
    }

    #[test]
    fn failure_is_always_failure() {
        for verify in [false, true] {
            for has_rdap in [false, true] {
                assert_eq!(
                    decide(&failure(), verify, has_rdap),
                    Decision::Report("failure")
                );
            }
        }
    }

    #[test]
    fn nxdomain_is_unconfirmed_without_verify() {
        // The bug: a held domain answers NXDOMAIN. We must not call it available.
        assert_eq!(
            decide(&available(), false, true),
            Decision::Report("unconfirmed")
        );
        assert_eq!(
            decide(&available(), false, false),
            Decision::Report("unconfirmed")
        );
    }

    #[test]
    fn nodata_is_unconfirmed_without_verify() {
        assert_eq!(
            decide(&nodata(), false, true),
            Decision::Report("unconfirmed")
        );
        assert_eq!(
            decide(&nodata(), false, false),
            Decision::Report("unconfirmed")
        );
    }

    #[test]
    fn verify_confirms_only_when_rdap_available() {
        assert_eq!(decide(&available(), true, true), Decision::Confirm);
        assert_eq!(decide(&nodata(), true, true), Decision::Confirm);
        // No RDAP endpoint for the zone: nothing to confirm against.
        assert_eq!(
            decide(&available(), true, false),
            Decision::Report("unconfirmed")
        );
        assert_eq!(
            decide(&nodata(), true, false),
            Decision::Report("unconfirmed")
        );
    }

    #[test]
    fn rdap_404_is_the_only_path_to_available() {
        let v = combine_rdap(
            "NXDOMAIN via ns1",
            RdapOutcome::Available {
                detail: "RDAP 404 via https://r".to_string(),
            },
        );
        assert_eq!(v.kind, "available");
    }

    #[test]
    fn rdap_200_catches_held_domains_as_registered() {
        let v = combine_rdap(
            "NXDOMAIN via ns1",
            RdapOutcome::Registered {
                detail: "RDAP 200 via https://r".to_string(),
            },
        );
        assert_eq!(v.kind, "registered");
    }

    #[test]
    fn rdap_429_stays_unconfirmed_and_is_legible() {
        let v = combine_rdap(
            "NXDOMAIN via ns1",
            RdapOutcome::RateLimited {
                detail: "RDAP 429 (retry-after 60s) via https://r".to_string(),
                retry_after: Some(Duration::from_secs(60)),
            },
        );
        assert_eq!(v.kind, "unconfirmed");
        assert!(v.detail.contains("RDAP 429"));
    }

    #[test]
    fn rdap_network_error_stays_unconfirmed() {
        let v = combine_rdap(
            "NXDOMAIN via ns1",
            RdapOutcome::Inconclusive {
                detail: "RDAP request failed via https://r: timed out".to_string(),
            },
        );
        assert_eq!(v.kind, "unconfirmed");
    }
}
