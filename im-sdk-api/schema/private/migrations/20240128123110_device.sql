-- Add migration script here
CREATE TABLE device (
    device_id INTEGER NOT NULL,
    public_key TEXT NOT NULL,
    server_public_key TEXT NOT NULL,
    server_private_key TEXT NOT NULL,
    def TEXT NOT NULL,
    user_id TEXT NOT NULL,
    token TEXT NOT NULL,
    proof TEXT NOT NULL,
    version TEXT NOT NULL,
    ext TEXT,
    last_ip TEXT NOT NULL,
    last_time INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    PRIMARY KEY (device_id)
);
CREATE INDEX im_device_id_idx ON device (device_id);