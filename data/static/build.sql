CREATE TABLE IF NOT EXISTS config (
    id bigint NOT NULL PRIMARY KEY,
    logging boolean NOT NULL DEFAULT false,
    log_channel_id bigint
);
