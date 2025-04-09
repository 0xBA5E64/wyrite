-- Add migration script here
CREATE TABLE posts (
    title TEXT,
    body TEXT,
    is_published INTEGER,
    date_created INTEGER
)