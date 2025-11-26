-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "unaccent";

CREATE OR REPLACE FUNCTION slugify(input TEXT) returns TEXT
  as $$ SELECT trim(regexp_replace(lower(unaccent(input)), '[^a-z0-9]+', '-', 'g'), '-'); $$ language SQL;

-- stolen from: https://dev.to/g5l/creating-unique-slugs-on-supabase-with-postgresql-3923
CREATE OR REPLACE FUNCTION public.set_unique_title_slug() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    base_slug TEXT;
    new_slug TEXT;
    counter INTEGER := 1;
BEGIN
-- Generate the base slug
    base_slug := slugify(NEW.title);
    new_slug := base_slug;

-- Check if the slug already exists
    WHILE EXISTS (SELECT 1 FROM posts WHERE slug = new_slug) LOOP
-- If it exists, append a number and increment
        new_slug := base_slug || '-' || counter;
        counter := counter + 1;
    END LOOP;

    NEW.slug := new_slug;
    RETURN NEW;
END
$$;

ALTER table posts add IF NOT EXISTS slug TEXT unique;

CREATE OR REPLACE TRIGGER "generate_slug" BEFORE INSERT ON "posts" FOR EACH row EXECUTE PROCEDURE set_unique_title_slug();

CREATE OR REPLACE TRIGGER "regenerate_slug" BEFORE UPDATE ON "posts" FOR EACH ROW EXECUTE PROCEDURE set_unique_title_slug();

UPDATE posts SET title=title WHERE slug IS NULL;

alter table POSTS alter slug set not null;

DROP VIEW post_view;
CREATE OR REPLACE VIEW post_view AS
    SELECT
        uuid AS "uuid!",
        slug AS "slug!",
        title AS "title!",
        body AS "body!",
        published IS NOT NULL AS "is_published!",
        uuid_extract_timestamp(uuid) AS "date_created!",
        published AS "date_published"
      FROM posts
      ORDER BY uuid_extract_timestamp(uuid) ASC;