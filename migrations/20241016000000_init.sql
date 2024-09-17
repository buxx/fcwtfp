CREATE TABLE IF NOT EXISTS session (
    key TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS session_member (
    session_key TEXT NOT NULL,
    name TEXT NOT NULL,
    discord_id TEXT,
    PRIMARY KEY (session_key, name),
    FOREIGN KEY(session_key) REFERENCES session(key) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS session_tech (
    session_key TEXT NOT NULL,
    session_member_name TEXT NOT NULL,
    name TEXT NOT NULL,
    done BOOLEAN NOT NULL CHECK (done IN (0, 1)),
    PRIMARY KEY (session_key, session_member_name, name),
    FOREIGN KEY(session_key, session_member_name) REFERENCES session_member(session_key, name) ON DELETE CASCADE
);