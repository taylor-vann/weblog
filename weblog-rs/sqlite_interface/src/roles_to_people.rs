use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct RoleToPerson {}

pub struct RolesToPeople {}

impl RolesToPeople {
    pub fn new() -> RolesToPeople {
        RolesToPeople {}
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
        "CREATE TABLE IF NOT EXISTS roles_to_people (
            id BIGINT PRIMARY KEY,
            role_id TEXT NOT NULL,
            people_id BIGINT NOT NULL,
            belongs_to BIGINT NOT NULL,
            deleted_at BIGINT,
        )",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        println!("error creating roles_to_people table")
    }
}

// // CREATE
// "
// INSERT INTO roles_to_people
// 	(id, kind)
// VALUES
// 	(?1, ?2);
// "

// // READ by people_id AND role_id
// "
// SELECT * FROM roles_to_people
// WHERE people_id = ?1 AND role_id = ?2;
// "
