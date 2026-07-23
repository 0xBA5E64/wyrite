-- Add down migration script here
CREATE VIEW post_view AS
    SELECT uuid AS "uuid!",
        slug AS "slug!",
        title AS "title!",
        body AS "body!",
        published IS NOT NULL AS "is_published!",
        uuid_extract_timestamp(uuid) AS "date_created!",
        published AS date_published
    FROM posts
    ORDER BY (uuid_extract_timestamp(uuid));
