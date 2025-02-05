use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct IpRateLimit {}

pub struct IpRateLimits {}

impl IpRateLimits {
    pub fn new() -> IpRateLimits {
        IpRateLimits {}
    }
    // create
    // read
    // read by email
    // update (email, password)
    // delete
}

pub fn create_table(path: &PathBuf) {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return,
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS ip_rate_limits {
            ip TEXT NOT NULL PRIMARY KEY,
            bucket_prev INTEGER NOT NULL,
            bucket_curr INTEGER NOT NULL,
            updated_at INTEGER,
            deleted_at INTEGER
        }",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        println!("error creating roles table")
    }
}
