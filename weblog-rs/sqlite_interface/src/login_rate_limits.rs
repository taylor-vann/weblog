use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct IpRateLimit {}

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (ip_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS ip_rate_limits (
            people_id INTEGER NOT NULL PRIMARY KEY UNIQUE,
            bucket_prev INTEGER NOT NULL,
            bucket_curr INTEGER NOT NULL,
            updated_at INTEGER,
            deleted_at INTEGER
    )",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        return Err("ip_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}
