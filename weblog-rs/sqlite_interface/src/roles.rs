pub struct Roles {}

// "
// CREATE TABLE IF NOT EXISTS roles (
// 	id BIGINT PRIMARY KEY,
// 	name TEXT NOT NULL UNIQUE,
// 	belongs_to BIGINT NOT NULL,
// 	deleted_at BIGINT,
// );
// "

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
