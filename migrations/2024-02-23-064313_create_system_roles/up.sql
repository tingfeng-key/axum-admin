-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_roles (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    -- sign VARCHAR NOT NULL UNIQUE,
    sort INTEGER NOT NULL DEFAULT 0,
    describe VARCHAR NOT NULL DEFAULT '',
    status INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);