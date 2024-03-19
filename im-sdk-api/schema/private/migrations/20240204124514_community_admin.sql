-- Add migration script here
CREATE TABLE IF NOT EXISTS community_admin (
    id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    FOREIGN KEY (community_id) REFERENCES community(id),
    FOREIGN KEY (user_id) REFERENCES account(user_id),
    PRIMARY KEY (id)
);
CREATE INDEX im_community_admin_id_idx ON community_admin (id);