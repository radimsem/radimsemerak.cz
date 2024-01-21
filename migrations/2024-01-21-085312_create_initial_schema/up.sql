CREATE TABLE IF NOT EXISTS tokens (
    id          SERIAL PRIMARY KEY,
    content     TEXT NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    expires     TIMESTAMP NOT NULL
);