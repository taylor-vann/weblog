use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Role {}

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS emails (
            id INTEGER PRIMARY KEY,
			email TEXT NOT NULL KEY UNIQUE,
			people_id INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        return Err("emails_to_emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(path: &PathBuf, email_id: u64, email: &str) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "INSERT OR IGNORE INTO emails
            (id, email)
        VALUES
            (?1, ?2)",
        (email_id, email),
    );

    if let Err(e) = results {
        return Err("emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, email_id: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "SELECT emails
        WHERE id = ?1",
        [email_id],
    );

    // iterate through emails

    if let Err(e) = results {
        return Err("read emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read_by_email(path: &PathBuf, email: &str) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "SELECT emails
        WHERE email = ?1",
        [email],
    );

    // iterate through emails

    if let Err(e) = results {
        return Err("read emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, email_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "UPDATE emails
        SET deleted_at = ?1
        WHERE id = ?2",
        (timestamp_ms, email_id),
    );

    if let Err(e) = results {
        return Err("delete emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(path: &PathBuf, email_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "DELETE emails
        WHERE id = ?1",
        [email_id],
    );

    if let Err(e) = results {
        return Err("dangerously delete emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}
