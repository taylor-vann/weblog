pub struct Session {}

// "
// CREATE TABLE IF NOT EXISTS sessions (
// 	id INTEGER PRIMARY KEY,
// 	session INTEGER NOT NULL,
// 	session_length_ms INTEGER NOT NULL,
// 	belongs_to INTEGER NOT NULL,
// 	updated_at INTEGER,
// 	deleted_at INTEGER
// );
// "

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
