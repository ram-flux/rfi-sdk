-- Add migration script here
CREATE TABLE IF NOT EXISTS community_post (
    id INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    content TEXT NOT NULL,
    sort_count INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    PRIMARY KEY (id),
    FOREIGN KEY (community_id) REFERENCES community(id),
    FOREIGN KEY (user_id) REFERENCES account(user_id)
);
CREATE INDEX im_community_post_id_idx ON community_post (id);
