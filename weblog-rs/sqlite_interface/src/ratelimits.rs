"
CREATE TABLE IF NOT EXISTS ip-session_ratelimits {
	ip TEXT NOT NULL PRIMARY KEY,
	bucket_prev INTEGER NOT NULL,
	bucket_curr INTEGER NOT NULL,
	updated_at INTEGER,
	deleted_at INTEGER
}
"


"
CREATE TABLE IF NOT EXISTS session_ratelimits {
	session_id INTEGER NOT NULL PRIMARY KEY,
	bucket_prev INTEGER NOT NULL,
	bucket_curr INTEGER NOT NULL,
	updated_at INTEGER,
	deleted_at INTEGER
}
"

// number of times an ip can create a session
// aka number of clients per minute
// 	16?
