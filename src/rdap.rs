// ABOUTME: Async RDAP probe used to disambiguate NODATA responses from compact-answer registries.
// ABOUTME: 200 means registered; 404 means available; anything else is inconclusive (None).

use std::time::Duration;

use reqwest::Client;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RdapOutcome {
    Registered { detail: String },
    Available { detail: String },
}

pub fn build_client(timeout: Duration) -> Result<Client, reqwest::Error> {
    // rustls 0.23 with rustls-no-provider needs an explicit provider install.
    // The call is idempotent across the process; ignoring Err lets repeated calls succeed.
    let _ = rustls::crypto::ring::default_provider().install_default();
    Client::builder()
        .timeout(timeout)
        .user_agent("vacant/0.1")
        .build()
}

pub async fn lookup(client: &Client, registered: &str, base_url: &str) -> Option<RdapOutcome> {
    let url = format!("{}/domain/{}", base_url.trim_end_matches('/'), registered);
    let response = client
        .get(&url)
        .header(reqwest::header::ACCEPT, "application/rdap+json")
        .send()
        .await
        .ok()?;
    let status = response.status();
    if status.is_success() {
        return Some(RdapOutcome::Registered {
            detail: format!("RDAP {} via {}", status.as_u16(), base_url),
        });
    }
    if status.as_u16() == 404 {
        return Some(RdapOutcome::Available {
            detail: format!("RDAP 404 via {}", base_url),
        });
    }
    None
}
