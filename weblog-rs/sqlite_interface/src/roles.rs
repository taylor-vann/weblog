"
CREATE TABLE IF NOT EXISTS roles (
	id BIGINT PRIMARY KEY,
	kind TEXT NOT NULL UNIQUE,
	belongs_to BIGINT NOT NULL,
	deleted_at BIGINT,
);
"

// CREATE
"
INSERT INTO roles
	(id, kind)
VALUES
	(?1, ?2);
"

// READ by kind
"
SELECT * FROM roles
WHERE kind = ?1;
"

// READ by id
"
SELECT * FROM roles
WHERE id = ?1;
"
