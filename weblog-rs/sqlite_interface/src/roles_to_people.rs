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

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (roles_to_people)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS roles_to_people (
            id INTEGER PRIMARY KEY,
            role_id TEXT NOT NULL,
            people_id INTEGER NOT NULL,
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
