-- Add migration script here
CREATE TABLE IF NOT EXISTS community_post_reply (
    id INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    sort INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    PRIMARY KEY (id),
    FOREIGN KEY (community_id) REFERENCES community(id),
    FOREIGN KEY (user_id) REFERENCES account(user_id),
    FOREIGN KEY (post_id) REFERENCES community_post(id)
);
CREATE INDEX im_community_post_reply_id_idx ON community_post_reply (id);