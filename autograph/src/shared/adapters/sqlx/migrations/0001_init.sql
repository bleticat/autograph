CREATE TABLE IF NOT EXISTS projects (
    id    TEXT PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cards (
    id         TEXT PRIMARY KEY,
    title      TEXT NOT NULL,
    completed  INTEGER NOT NULL DEFAULT 0,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL
);
