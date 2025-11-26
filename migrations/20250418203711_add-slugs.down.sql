-- Add down migration script here
DROP VIEW post_view;
CREATE OR REPLACE VIEW post_view AS
    SELECT
        uuid AS "uuid!",
        title AS "title!",
        body AS "body!",
        published IS NOT NULL AS "is_published!",
        uuid_extract_timestamp(uuid) AS "date_created!",
        published AS "date_published"
      FROM posts
      ORDER BY uuid_extract_timestamp(uuid) ASC;
ALTER TABLE posts DROP COLUMN slug;
DROP TRIGGER "generate_slug" ON posts;
DROP TRIGGER "regenerate_slug" ON posts;
DROP FUNCTION set_unique_title_slug();
DROP FUNCTION slugify(TEXT);
DROP EXTENSION "unaccent";
