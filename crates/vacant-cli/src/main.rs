// ABOUTME: vacant — fast domain availability checker via authoritative DNS NS lookups.
// ABOUTME: Mirrors the python check command - same flags, same JSONL/text/table output shape.

use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use clap::{Parser, ValueEnum};
use vacant_core::{check_many, CheckResult, DiskCache, DnsClient, RuleSet, Status};
use serde::Serialize;

const BUNDLED_RULES: &str = include_str!("../data/rules.toml");

#[derive(Parser, Debug)]
#[command(name = "vacant", about = "Check domain availability via authoritative DNS.")]
struct Cli {
    /// Domains to check; '-' or no args reads stdin (one per line).
    #[arg(value_name = "DOMAIN")]
    domains: Vec<String>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Jsonl)]
    output: OutputFormat,

    /// Concurrent in-flight queries.
    #[arg(long, default_value_t = 64)]
    concurrency: usize,

    /// Per-query timeout in seconds.
    #[arg(long, default_value_t = 4.0)]
    timeout: f64,

    /// Include the zone, raw input, detail, and cached fields in output.
    #[arg(long)]
    detail: bool,

    /// Disable on-disk result cache.
    #[arg(long)]
    no_cache: bool,

    /// Disk cache TTL in seconds.
    #[arg(long, default_value_t = 86_400.0)]
    cache_ttl: f64,

    /// Override cache database path.
    #[arg(long)]
    cache_path: Option<PathBuf>,

    /// Override the bundled rules.toml path.
    #[arg(long, env = "VACANT_RULES")]
    rules: Option<PathBuf>,

    // status filters - additive; none means "all"
    #[arg(long)] available: bool,
    #[arg(long)] registered: bool,
    #[arg(long)] reserved: bool,
    #[arg(long)] invalid: bool,
    #[arg(long)] unknown: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Jsonl,
    Table,
    Text,
}

#[derive(Debug, Serialize)]
struct OutResult<'a> {
    input: &'a str,
    domain: &'a str,
    zone: &'a str,
    status: &'static str,
    detail: &'a str,
    from_cache: bool,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("error: {e}");
        std::process::exit(2);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let rules_text = match &cli.rules {
        Some(path) => std::fs::read_to_string(path)?,
        None => BUNDLED_RULES.to_string(),
    };
    let rules = RuleSet::from_str(&rules_text)?;

    let cache = if cli.no_cache {
        None
    } else {
        let path = cli.cache_path.clone().unwrap_or_else(DiskCache::default_path);
        Some(DiskCache::open(&path)?)
    };

    let dns = DnsClient::new(Duration::from_secs_f64(cli.timeout.max(0.05)))?;

    let inputs = collect_inputs(&cli.domains)?;

    let results = check_many(
        &rules,
        &dns,
        cache.as_ref(),
        &inputs,
        cli.cache_ttl as i64,
        cli.concurrency,
    );

    let filter = StatusFilter::from_cli(&cli);
    let final_results: Vec<&CheckResult> = results
        .iter()
        .filter(|r| filter.allows(r.status))
        .collect();

    emit(&final_results, cli.output, cli.detail)?;

    if final_results.iter().any(|r| matches!(r.status, Status::Unknown)) {
        std::process::exit(2);
    }
    Ok(())
}

struct StatusFilter {
    allowed: Vec<Status>,
}

impl StatusFilter {
    fn from_cli(cli: &Cli) -> Self {
        let mut allowed = Vec::new();
        if cli.available { allowed.push(Status::Available); }
        if cli.registered { allowed.push(Status::Registered); }
        if cli.reserved { allowed.push(Status::Reserved); }
        if cli.invalid { allowed.push(Status::Invalid); }
        if cli.unknown { allowed.push(Status::Unknown); }
        Self { allowed }
    }
    fn allows(&self, status: Status) -> bool {
        self.allowed.is_empty() || self.allowed.contains(&status)
    }
}

fn collect_inputs(args: &[String]) -> io::Result<Vec<String>> {
    if args.is_empty() || args.iter().any(|a| a == "-") {
        let mut out: Vec<String> = args.iter().filter(|a| *a != "-").cloned().collect();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let s = line.trim();
            if s.is_empty() || s.starts_with('#') { continue; }
            out.push(s.to_string());
        }
        return Ok(out);
    }
    Ok(args.to_vec())
}

fn as_out<'a>(r: &'a CheckResult) -> OutResult<'a> {
    OutResult {
        input: &r.input,
        domain: &r.domain,
        zone: &r.zone,
        status: r.status.as_str(),
        detail: &r.detail,
        from_cache: r.from_cache,
    }
}

fn emit(results: &[&CheckResult], format: OutputFormat, detail: bool) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match format {
        OutputFormat::Jsonl => {
            for r in results {
                let value = if detail {
                    serde_json::to_value(as_out(r)).unwrap()
                } else {
                    serde_json::json!({"domain": r.domain, "status": r.status.as_str()})
                };
                writeln!(out, "{}", serde_json::to_string(&value).unwrap())?;
            }
        }
        OutputFormat::Text => {
            for r in results {
                writeln!(out, "{}", if r.domain.is_empty() { &r.input } else { &r.domain })?;
            }
        }
        OutputFormat::Table => {
            if detail {
                writeln!(out, "{:<40} {:<10} {:<10} {:<6} {}", "domain", "zone", "status", "cache", "detail")?;
                writeln!(out, "{}", "-".repeat(100))?;
                for r in results {
                    writeln!(out, "{:<40} {:<10} {:<10} {:<6} {}",
                        r.domain, r.zone, r.status.as_str(),
                        if r.from_cache { "yes" } else { "" }, r.detail)?;
                }
            } else {
                writeln!(out, "{:<40} {}", "domain", "status")?;
                writeln!(out, "{}", "-".repeat(60))?;
                for r in results {
                    writeln!(out, "{:<40} {}", r.domain, r.status.as_str())?;
                }
            }
        }
    }
    Ok(())
}
