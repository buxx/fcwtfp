CREATE TABLE IF NOT EXISTS session (
    key TEXT NOT NULL,
    discord_id TEXT NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (key)
);

CREATE INDEX idx_session_discord_id ON session (discord_id);

CREATE TABLE IF NOT EXISTS session_member (
    session_key TEXT NOT NULL,
    discord_id TEXT NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (session_key, discord_id),
    FOREIGN KEY(session_key) REFERENCES session(key) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS session_tech (
    session_key TEXT NOT NULL,
    session_member_discord_id TEXT NOT NULL,
    name TEXT NOT NULL,
    done BOOLEAN NOT NULL CHECK (done IN (0, 1)),
    PRIMARY KEY (session_key, session_member_discord_id, name),
    FOREIGN KEY(session_key, session_member_discord_id) REFERENCES session_member(session_key, discord_id) ON DELETE CASCADE
);