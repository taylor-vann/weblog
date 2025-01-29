"CREATE TABLE IF NOT EXISTS people (
	id INTEGER PRIMARY KEY,
	email TEXT NOT NULL UNIQUE,
	screen_name TEXT NOT NULL UNIQUE,
	password_hash_params TEXT NOT NULL,
	updated_at INTEGER,
	deleted_at INTEGER
);"
