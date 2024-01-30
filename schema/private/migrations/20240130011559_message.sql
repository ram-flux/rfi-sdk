-- Add migration script here
CREATE TABLE message (
    id INTEGER NOT NULL,
    mode INTEGER NOT NULL,
    from_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    chat_id INTEGER NOT NULL,
    chat_type INTEGER NOT NULL,
    datas TEXT NOT NULL,
    has_read INTEGER NOT NULL,
    msg_error INTEGER NOT NULL,
    status INTEGER NOT NULL,
    send_time TEXT NOT NULL,
    accept_time TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE INDEX im_message_id_idx ON device (device_id);