-- Add migration script here
CREATE TABLE IF NOT EXISTS community_member (
    id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    avatar TEXT NOT NULL,
    sort INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    PRIMARY KEY (id),
    CONSTRAINT community_member_pkey UNIQUE (user_id, community_id),
    FOREIGN KEY (user_id) REFERENCES account(user_id),
    FOREIGN KEY (community_id) REFERENCES community(id)
);
CREATE INDEX im_community_member_id_idx ON community_member (id);
CREATE INDEX im_community_member_user_id_community_id_idx ON community_member (user_id, community_id);