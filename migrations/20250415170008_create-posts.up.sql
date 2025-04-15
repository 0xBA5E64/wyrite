-- Add up migration script here
-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- this gives us the ability to generate UUIDv1's, with timestamps & MAC's

CREATE TABLE posts (
    uuid UUID DEFAULT uuid_generate_v1() PRIMARY KEY UNIQUE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    published TIMESTAMPTZ DEFAULT NULL
);

CREATE VIEW post_view AS
    SELECT
        uuid,
        title,
        body,
        published IS NOT NULL AS is_published,
        uuid_extract_timestamp(uuid) AS date_created,
        published AS date_published
      FROM posts
      ORDER BY uuid_extract_timestamp(uuid) ASC;