const UTC_JAN_1ST_2025_MS = 1735689600000;

"
CREATE TABLE IF NOT EXISTS sessions-people (
	id INTEGER PRIMARY KEY,
	session INTEGER NOT NULL,
	session_length_ms INTEGER NOT NULL,
	two_factor_auth_id INTEGER,
	belongs_to INTEGER,
	updated_at INTEGER,
	deleted_at INTEGER
);
"
