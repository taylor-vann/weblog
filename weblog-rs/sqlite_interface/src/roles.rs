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

pub fn create_table(path: &PathBuf) {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return,
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS roles (
            id BIGINT PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            belongs_to BIGINT NOT NULL,
            deleted_at BIGINT,
        );",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        println!("error creating roles table")
    }
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
