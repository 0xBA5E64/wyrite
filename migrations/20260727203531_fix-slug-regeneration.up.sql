-- Add up migration script here

-- Previous trigger would update even if you didn't change the slug,
-- -leading to slugs getting suffixed with -1 when other fields were changed.
CREATE OR REPLACE TRIGGER "regenerate_slug" BEFORE UPDATE OF slug ON "posts" FOR EACH ROW EXECUTE PROCEDURE set_unique_title_slug();
