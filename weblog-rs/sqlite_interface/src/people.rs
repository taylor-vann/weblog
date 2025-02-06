use rusqlite::{Connection, Result};
use std::path::PathBuf;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

pub struct Person {}

pub struct People {}

impl People {
    pub fn new() -> People {
        People {}
    }

    fn create(name: String, password: String) -> Result<(), String> {
        // sdf
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(ph) => ph.to_string(),
            Err(e) => return Err("person, create error:\n".to_string() + &e.to_string()),
        };

        println!("{:?}", password_hash);
        Ok(())
    }

    // create
    // read
    // read by email
    // update (email, password)
    // delete
}

// keep table creation out of regular api?
pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
			id INTEGER PRIMARY KEY,
			email TEXT NOT NULL UNIQUE,
			screen_name TEXT NOT NULL UNIQUE,
			password_hash_params TEXT NOT NULL,
			updated_at INTEGER,
			deleted_at INTEGER
		)",
        (), // empty list of parameters.
    );

    if let Err(e) = results {
        return Err("people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

// "CREATE TABLE IF NOT EXISTS people (
// 	id INTEGER PRIMARY KEY,
// 	email TEXT NOT NULL UNIQUE,
// 	password_hash_params TEXT NOT NULL,
// 	updated_at INTEGER,
// 	deleted_at INTEGER
// );"

// // CREATE
// "INSERT INTO people
// 	(id, email, password_hash_params)
// VALUES
// 	(?1, ?2, ?3);
// "

// // READ by id
// "
// SELECT * FROM people
// WHERE id = ?1;
// "

// // READ by email
// "
// SELECT * FROM people
// WHERE email = ?1;
// "

// // INDEPENDENT UPDATES
// //   intention is to avoid accidentally changing key user info
// //
// // update email
// // update screen_name
// // update password_hash_parmas
// //

// // DELETE
