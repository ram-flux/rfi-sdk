-- Add migration script here
CREATE TABLE account (
    user_id INTEGER NOT NULL,
    public_key TEXT NOT NULL,
    account TEXT NOT NULL,
    salt TEXT NOT NULL,
    gender INTEGER NOT NULL,
    name TEXT NOT NULL,
    avatar TEXT NOT NULL,
    bio TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    PRIMARY KEY (user_id)
);

CREATE INDEX im_account_id_idx ON account (user_id);