use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Session {}

pub struct Sessions {}

impl Sessions {
    pub fn new() -> Sessions {
        Sessions {}
    }
    // create
    // read
    // read by email
    // update (email, password)
    // delete
}

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            session INTEGER NOT NULL,
            session_length_ms INTEGER NOT NULL,
            belongs_to INTEGER,
            deleted_at INTEGER
        )",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        return Err("sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}

// // CREATE
// "
// INSERT INTO sessions
// 	(id, session, session_length_ms, belongs_to)
// VALUES
// 	(?1, ?2, ?3, ?4);
// "

// // READ by id
// "
// SELECT * FROM sessions
// WHERE id = ?1;
// "
