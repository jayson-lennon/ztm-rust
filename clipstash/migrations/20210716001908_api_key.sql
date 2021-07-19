-- Add migration script here
CREATE TABLE IF NOT EXISTS api_keys
(
    api_key BLOB PRIMARY KEY
);
