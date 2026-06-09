// ABOUTME: MCP stdio server exposing vacant's domain checks as one read-only tool.
// ABOUTME: Mirrors python/vacant/mcp.py — check_domains(domains, verify=false) over check_many.

use std::sync::Arc;
use std::time::Duration;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};
use rmcp::schemars::{self, JsonSchema};
use rmcp::transport::stdio;
use rmcp::{tool, tool_handler, tool_router, Json, ServerHandler, ServiceExt};
use serde::{Deserialize, Serialize};

use crate::{check_many, DnsClient, RuleSet};

const MAX_DOMAINS: usize = 500;
const TIMEOUT_SECS: f64 = 4.0;
const CONCURRENCY: usize = 64;

struct Engine {
    rules: RuleSet,
    dns: DnsClient,
}

#[derive(Deserialize, JsonSchema)]
struct CheckDomainsArgs {
    /// Domain names to check. Blank entries and ones starting with '#' are skipped.
    domains: Vec<String>,
    /// Confirm undelegated names against the registry (RDAP) so they resolve to
    /// 'available' or 'registered' rather than 'unconfirmed'.
    #[serde(default)]
    verify: bool,
}

#[derive(Serialize, JsonSchema)]
struct DomainStatus {
    domain: String,
    status: String,
}

/// MCP requires a tool's structured output to have an object root, so the
/// per-input list is nested under `result` — matching the Python server.
#[derive(Serialize, JsonSchema)]
struct CheckDomainsResult {
    result: Vec<DomainStatus>,
}

#[derive(Clone)]
struct VacantServer {
    engine: Arc<Engine>,
    // Read by the #[tool_handler]-generated routing code, which the dead-code
    // lint can't see through.
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl VacantServer {
    fn new(engine: Arc<Engine>) -> Self {
        Self {
            engine,
            tool_router: Self::tool_router(),
        }
    }

    /// Check domain availability across any TLD via authoritative DNS.
    ///
    /// Pass one name or many; returns one {domain, status} per input, in order.
    /// status is one of: registered, available, unconfirmed, reserved, invalid, unknown.
    ///
    /// 'available' only ever appears when verify=true, which confirms undelegated
    /// names against the registry (RDAP). Without it, a name with no DNS delegation
    /// reports 'unconfirmed' (probably free, but a held or expired domain looks the
    /// same to DNS). Set verify=true when the user asks whether a specific name is
    /// free; leave it false to cheaply screen a large list down to candidates.
    #[tool]
    async fn check_domains(
        &self,
        Parameters(args): Parameters<CheckDomainsArgs>,
    ) -> Json<CheckDomainsResult> {
        let engine = self.engine.clone();
        // check_many drives its own runtime via block_on; keep it off the async
        // worker threads by running it on the blocking pool.
        let result = tokio::task::spawn_blocking(move || {
            let cleaned: Vec<String> = args
                .domains
                .iter()
                .map(|d| d.trim())
                .filter(|d| !d.is_empty() && !d.starts_with('#'))
                .take(MAX_DOMAINS)
                .map(str::to_string)
                .collect();
            let results = check_many(
                &engine.rules,
                &engine.dns,
                None,
                &cleaned,
                0,
                CONCURRENCY,
                args.verify,
            );
            results
                .into_iter()
                .map(|r| DomainStatus {
                    domain: if r.domain.is_empty() {
                        r.input
                    } else {
                        r.domain
                    },
                    status: r.status.as_str().to_string(),
                })
                .collect::<Vec<_>>()
        })
        .await
        .unwrap_or_default();
        Json(CheckDomainsResult { result })
    }
}

#[tool_handler]
impl ServerHandler for VacantServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("vacant", env!("CARGO_PKG_VERSION")))
            .with_instructions("Check domain availability across any TLD via authoritative DNS.")
    }
}

pub fn serve(rules: RuleSet) -> Result<(), Box<dyn std::error::Error>> {
    let dns = DnsClient::new(Duration::from_secs_f64(TIMEOUT_SECS))?;
    // DnsClient owns its own tokio runtime. Holding this Arc until serve()
    // returns keeps the engine (and that runtime) from being dropped inside the
    // serving runtime's async context, which tokio forbids.
    let engine = Arc::new(Engine { rules, dns });
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let service = VacantServer::new(engine.clone()).serve(stdio()).await?;
        service.waiting().await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}
