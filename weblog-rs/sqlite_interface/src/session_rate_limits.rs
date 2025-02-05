use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct SessionRateLimit {}

pub struct SessionRateLimits {}

impl SessionRateLimits {
    pub fn new() -> SessionRateLimits {
        SessionRateLimits {}
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
        "CREATE TABLE IF NOT EXISTS session_rate_limits (
            session_id INTEGER NOT NULL PRIMARY KEY,
            bucket_prev INTEGER NOT NULL,
            bucket_curr INTEGER NOT NULL,
            updated_at INTEGER,
            deleted_at INTEGER
        )",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        println!("error creating roles table")
    }
}

// // number of times an ip can create a session
// // aka number of clients per minute
// // 	16?
// // CREATE
// "
// INSERT INTO session_Sessionratelimits
// 	(session_id, bucket_prev, bucket_prev)
// VALUES
// 	(?1, ?2, ?3);
// "

// // READ by id
// "
// SELECT * FROM session_Sessionratelimits
// WHERE session_id = ?1;
// "

// // UPDATE
// "
// INSERT INTO session_Sessionratelimits
// 	(bucket_prev, bucket_prev, updated_at)
// VALUES
// 	(?1, ?2, ?3)
// WHERE id = ?4;
// "
