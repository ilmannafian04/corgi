CREATE TABLE links (
    id serial NOT NULL PRIMARY KEY,
    shortened VARCHAR NOT NULL UNIQUE,
    original TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
)