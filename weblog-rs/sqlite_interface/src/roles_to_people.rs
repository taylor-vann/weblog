pub struct RolesToPeople {}

// "CREATE TABLE IF NOT EXISTS roles_to_people (
// 	id BIGINT PRIMARY KEY,
// 	role_id TEXT NOT NULL,
// 	people_id BIGINT NOT NULL,
// 	belongs_to BIGINT NOT NULL,
// 	deleted_at BIGINT,
// )"

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
