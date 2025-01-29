"CREATE TABLE IF NOT EXISTS roles_people (
	id BIGINT PRIMARY KEY,
	role_id TEXT NOT NULL,
	people_id BIGINT NOT NULL,
	belongs_to BIGINT NOT NULL,
	deleted_at BIGINT,
)"
