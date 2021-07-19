-- Add migration script here
CREATE TABLE IF NOT EXISTS clips
(
    clip_id   TEXT PRIMARY KEY NOT NULL,
    shortcode TEXT UNIQUE NOT NULL,
    content   TEXT NOT NULL,
    title     TEXT,
    posted    DATETIME NOT NULL,
    expires   DATETIME,
    password  TEXT,
    hits      BIGINT NOT NULL
);
