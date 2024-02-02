-- Add migration script here
CREATE TABLE community (
    id INTEGER NOT NULL,
    father_id INTEGER,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    bio TEXT NOT NULL,
    passwd TEXT,
    announcement TEXT,
    pinned BOOLEAN NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE INDEX im_community_id_idx ON community (id);