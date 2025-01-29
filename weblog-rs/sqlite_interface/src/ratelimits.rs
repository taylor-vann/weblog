"
CREATE TABLE IF NOT EXISTS seession_creation_ratelimits {
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
// CREATE
"
INSERT INTO session_ratelimits
	(session_id, bucket_prev, bucket_prev)
VALUES
	(?1, ?2, ?3);
"

// READ by id
"
SELECT * FROM session_ratelimits
WHERE session_id = ?1;
"

// UPDATE
"
INSERT INTO session_ratelimits
	(bucket_prev, bucket_prev, updated_at)
VALUES
	(?1, ?2, ?3)
WHERE id = ?4;
"

