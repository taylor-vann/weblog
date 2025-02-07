use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Role {}

pub struct Roles {}

impl Roles {
    pub fn new() -> Roles {
        Roles {}
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
        Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            created_by INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        return Err("roles_to_people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

// // CREATE
// "
// INSERT INTO roles
// 	(id, name)
// VALUES
// 	(?1, ?2);
// "

// // READ by name
// "
// SELECT * FROM roles
// WHERE name = ?1;
// "

// // READ by id
// "
// SELECT * FROM roles
// WHERE id = ?1;
// "
