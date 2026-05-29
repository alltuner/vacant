// ABOUTME: napi-rs bindings exposing the vacant Rust engine to Node.js.
// ABOUTME: Surface: loadRules(), checkMany(), DiskCache. Lockstep-versioned with vacant.

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;

use napi::{Error, Result, Status};
use napi_derive::napi;
use vacant::{check_many as core_check_many, CheckResult, DiskCache, DnsClient, RuleSet};

static RULES: RwLock<Option<RuleSet>> = RwLock::new(None);
static DNS: OnceLock<DnsClient> = OnceLock::new();

fn dns_client(timeout: Duration) -> Result<&'static DnsClient> {
    if let Some(client) = DNS.get() {
        return Ok(client);
    }
    let client =
        DnsClient::new(timeout).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let _ = DNS.set(client);
    Ok(DNS.get().expect("DNS just set"))
}

#[napi(js_name = "loadRules")]
pub fn load_rules(path: String) -> Result<()> {
    let path = PathBuf::from(path);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| Error::new(Status::InvalidArg, format!("read {}: {e}", path.display())))?;
    let rs =
        RuleSet::from_str(&text).map_err(|e| Error::new(Status::InvalidArg, format!("{e}")))?;
    let mut guard = RULES
        .write()
        .map_err(|_| Error::new(Status::GenericFailure, "rules lock poisoned"))?;
    *guard = Some(rs);
    Ok(())
}

#[napi(object)]
pub struct CheckRow {
    pub input: String,
    pub domain: String,
    pub zone: String,
    pub status: String,
    pub detail: String,
    pub from_cache: bool,
}

impl From<CheckResult> for CheckRow {
    fn from(r: CheckResult) -> Self {
        CheckRow {
            input: r.input,
            domain: r.domain,
            zone: r.zone,
            status: r.status.as_str().to_string(),
            detail: r.detail,
            from_cache: r.from_cache,
        }
    }
}

#[napi(js_name = "checkMany")]
pub fn check_many(
    domains: Vec<String>,
    concurrency: Option<u32>,
    timeout: Option<f64>,
    cache: Option<&JsDiskCache>,
    cache_ttl: Option<f64>,
    verify: Option<bool>,
) -> Result<Vec<CheckRow>> {
    let concurrency = concurrency.unwrap_or(64) as usize;
    let timeout = timeout.unwrap_or(4.0);
    let cache_ttl = cache_ttl.unwrap_or(86_400.0);
    let verify = verify.unwrap_or(false);

    let dur = Duration::from_secs_f64(timeout.max(0.05));
    let dns = dns_client(dur)?;
    let cache_ref = cache.map(|c| &c.inner);

    let guard = RULES
        .read()
        .map_err(|_| Error::new(Status::GenericFailure, "rules lock poisoned"))?;
    let rules = guard.as_ref().ok_or_else(|| {
        Error::new(
            Status::GenericFailure,
            "rules not loaded; call loadRules() first",
        )
    })?;

    let results = core_check_many(
        rules,
        dns,
        cache_ref,
        &domains,
        cache_ttl as i64,
        concurrency,
        verify,
    );

    Ok(results.into_iter().map(CheckRow::from).collect())
}

#[napi(object)]
pub struct CachedRowJs {
    pub domain: String,
    pub zone: String,
    pub status: String,
    pub detail: String,
    pub checked_at: i64,
}

#[napi(js_name = "DiskCache")]
pub struct JsDiskCache {
    inner: DiskCache,
}

#[napi]
impl JsDiskCache {
    #[napi(constructor)]
    pub fn new(path: Option<String>) -> Result<Self> {
        let resolved = path
            .map(PathBuf::from)
            .unwrap_or_else(DiskCache::default_path);
        let inner = DiskCache::open(&resolved)
            .map_err(|e| Error::new(Status::InvalidArg, format!("{e}")))?;
        Ok(Self { inner })
    }

    #[napi(js_name = "defaultPath")]
    pub fn default_path_static() -> String {
        DiskCache::default_path().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn get(&self, domain: String, ttl: f64) -> Result<Option<CachedRowJs>> {
        let row = self
            .inner
            .get(&domain, ttl as i64)
            .map_err(|e| Error::new(Status::InvalidArg, format!("{e}")))?;
        Ok(row.map(|r| CachedRowJs {
            domain: r.domain,
            zone: r.zone,
            status: r.status,
            detail: r.detail,
            checked_at: r.checked_at,
        }))
    }

    #[napi]
    pub fn put(&self, domain: String, zone: String, status: String, detail: String) -> Result<()> {
        self.inner
            .put(&domain, &zone, &status, &detail)
            .map_err(|e| Error::new(Status::InvalidArg, format!("{e}")))
    }
}
