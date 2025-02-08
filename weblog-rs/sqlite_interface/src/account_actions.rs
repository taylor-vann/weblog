use rusqlite::{Connection, Result};
use std::path::PathBuf;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

// snowflake

// create_account
// update_password
// update_email

// keep table creation out of regular api?
pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (account_actions)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS account_actions (
			id INTEGER PRIMARY KEY,
			kind TEXT NOT NULL,
			deleted_at INTEGER
		)",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        return Err("account_actions: \n".to_string() + &e.to_string());
    }

    Ok(())
}
