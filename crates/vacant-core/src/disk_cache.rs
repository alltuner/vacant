// ABOUTME: SQLite-backed cache for check results.
// ABOUTME: WAL + NORMAL synchronous so concurrent tokio tasks can read/write without blocking each other.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use thiserror::Error;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS results (
    domain TEXT PRIMARY KEY,
    zone TEXT NOT NULL,
    status TEXT NOT NULL,
    detail TEXT NOT NULL DEFAULT '',
    checked_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_results_checked_at ON results(checked_at);
"#;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct CachedRow {
    pub domain: String,
    pub zone: String,
    pub status: String,
    pub detail: String,
    pub checked_at: i64,
}

pub struct DiskCache {
    conn: Mutex<Connection>,
}

impl DiskCache {
    pub fn open(path: &Path) -> Result<Self, CacheError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn default_path() -> PathBuf {
        let base = std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".cache")
            });
        base.join("vacant").join("results.db")
    }

    pub fn get(&self, domain: &str, ttl_secs: i64) -> Result<Option<CachedRow>, CacheError> {
        let conn = self.conn.lock().expect("disk cache lock");
        let mut stmt = conn.prepare_cached(
            "SELECT domain, zone, status, detail, checked_at FROM results WHERE domain = ?1",
        )?;
        let row = stmt
            .query_row(params![domain], |row| {
                Ok(CachedRow {
                    domain: row.get(0)?,
                    zone: row.get(1)?,
                    status: row.get(2)?,
                    detail: row.get(3)?,
                    checked_at: row.get(4)?,
                })
            })
            .ok();
        let Some(r) = row else { return Ok(None) };
        let now = current_unix();
        if now - r.checked_at > ttl_secs {
            return Ok(None);
        }
        Ok(Some(r))
    }

    pub fn put(
        &self,
        domain: &str,
        zone: &str,
        status: &str,
        detail: &str,
    ) -> Result<(), CacheError> {
        let conn = self.conn.lock().expect("disk cache lock");
        let mut stmt = conn.prepare_cached(
            "INSERT OR REPLACE INTO results(domain, zone, status, detail, checked_at) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        stmt.execute(params![domain, zone, status, detail, current_unix()])?;
        Ok(())
    }
}

fn current_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
