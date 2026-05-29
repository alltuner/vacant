// ABOUTME: Async RDAP probe used to disambiguate NODATA responses from compact-answer registries.
// ABOUTME: 200 means registered; 404 means available; 429 is retried politely; anything else is inconclusive.

use std::time::Duration;

use reqwest::{Client, StatusCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RdapOutcome {
    Registered {
        detail: String,
    },
    Available {
        detail: String,
    },
    /// Still 429 after our retry effort. `retry_after` is the server's hint, if
    /// any, used to size how long the host is left alone on subsequent runs.
    RateLimited {
        detail: String,
        retry_after: Option<Duration>,
    },
    Inconclusive {
        detail: String,
    },
}

/// Retry budget for RDAP 429s. Each retry waits for the server's `Retry-After`
/// when present, otherwise an exponentially backed-off, jittered delay. The cap
/// keeps a rate-limited host from stalling a whole batch.
const MAX_RETRIES: u32 = 3;
const BACKOFF_BASE: Duration = Duration::from_millis(500);
const BACKOFF_CAP: Duration = Duration::from_secs(8);

pub fn build_client(timeout: Duration) -> Result<Client, reqwest::Error> {
    // rustls 0.23 with rustls-no-provider needs an explicit provider install.
    // The call is idempotent across the process; ignoring Err lets repeated calls succeed.
    let _ = rustls::crypto::ring::default_provider().install_default();
    Client::builder()
        .timeout(timeout)
        .user_agent("vacant/0.1")
        .build()
}

pub async fn lookup(client: &Client, registered: &str, base_url: &str) -> RdapOutcome {
    let base = base_url.trim_end_matches('/');
    let url = format!("{base}/domain/{registered}");
    let mut attempt = 0;
    loop {
        let response = match client
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/rdap+json")
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return RdapOutcome::Inconclusive {
                    detail: format!("RDAP request failed via {base_url}: {e}"),
                }
            }
        };
        let status = response.status();
        if status.is_success() {
            return RdapOutcome::Registered {
                detail: format!("RDAP {} via {base_url}", status.as_u16()),
            };
        }
        if status == StatusCode::NOT_FOUND {
            return RdapOutcome::Available {
                detail: format!("RDAP 404 via {base_url}"),
            };
        }
        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(parse_retry_after);
            if should_retry_429(attempt, retry_after) {
                tokio::time::sleep(retry_delay(attempt, retry_after)).await;
                attempt += 1;
                continue;
            }
            // Out of budget, or the host shut us out for far longer than we'd
            // wait (e.g. a Cloudflare bot block with an hours-long Retry-After):
            // retrying in-run is futile, so surface the block and move on.
            return RdapOutcome::RateLimited {
                detail: match retry_after {
                    Some(after) => {
                        format!("RDAP 429 (retry-after {}s) via {base_url}", after.as_secs())
                    }
                    None => format!("RDAP 429 via {base_url}"),
                },
                retry_after,
            };
        }
        return RdapOutcome::Inconclusive {
            detail: format!("RDAP {} via {base_url}", status.as_u16()),
        };
    }
}

/// Whether a 429 is worth retrying: we still have budget, and the server hasn't
/// told us to wait far longer than we will. An hours-long `Retry-After` means
/// we've been shut out (e.g. a Cloudflare bot block), not throttled — retrying
/// in-run can't recover it.
fn should_retry_429(attempt: u32, retry_after: Option<Duration>) -> bool {
    attempt < MAX_RETRIES && retry_after.is_none_or(|after| after <= BACKOFF_CAP)
}

/// Parse the delta-seconds form of `Retry-After`. The HTTP-date form is rare for
/// RDAP and falls through to plain backoff.
fn parse_retry_after(value: &str) -> Option<Duration> {
    value.trim().parse::<u64>().ok().map(Duration::from_secs)
}

/// How long to wait before the next retry. Honour `Retry-After` when the server
/// sent one (capped), otherwise exponential backoff with half-window jitter.
fn retry_delay(attempt: u32, retry_after: Option<Duration>) -> Duration {
    if let Some(after) = retry_after {
        return after.min(BACKOFF_CAP);
    }
    let bounded = BACKOFF_BASE
        .saturating_mul(1u32 << attempt)
        .min(BACKOFF_CAP);
    bounded.mul_f64(0.5 + 0.5 * rand::random::<f64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_delta_seconds_retry_after() {
        assert_eq!(parse_retry_after("5"), Some(Duration::from_secs(5)));
        assert_eq!(parse_retry_after("  12 "), Some(Duration::from_secs(12)));
    }

    #[test]
    fn ignores_http_date_retry_after() {
        assert_eq!(parse_retry_after("Wed, 21 Oct 2015 07:28:00 GMT"), None);
        assert_eq!(parse_retry_after(""), None);
    }

    #[test]
    fn retries_429_within_budget_but_not_when_shut_out() {
        assert!(should_retry_429(0, None));
        assert!(should_retry_429(0, Some(Duration::from_secs(2))));
        // Budget spent.
        assert!(!should_retry_429(MAX_RETRIES, None));
        // Hours-long Retry-After: blocked, not throttled — don't bother.
        assert!(!should_retry_429(0, Some(Duration::from_secs(81_381))));
    }

    #[test]
    fn retry_after_is_honoured_and_capped() {
        assert_eq!(
            retry_delay(0, Some(Duration::from_secs(3))),
            Duration::from_secs(3)
        );
        // A huge Retry-After is clamped so one rude host can't stall the batch.
        assert_eq!(retry_delay(0, Some(Duration::from_secs(600))), BACKOFF_CAP);
    }

    #[test]
    fn backoff_grows_but_stays_within_bounds() {
        for attempt in 0..MAX_RETRIES {
            let window = BACKOFF_BASE
                .saturating_mul(1u32 << attempt)
                .min(BACKOFF_CAP);
            let d = retry_delay(attempt, None);
            // Half-window jitter: at least half the window, never more than it.
            assert!(d >= window.mul_f64(0.5));
            assert!(d <= window);
        }
    }
}
