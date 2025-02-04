pub struct People {}

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
