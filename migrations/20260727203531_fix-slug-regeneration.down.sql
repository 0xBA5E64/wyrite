-- Add down migration script here
CREATE OR REPLACE TRIGGER "regenerate_slug" BEFORE UPDATE ON "posts" FOR EACH ROW EXECUTE PROCEDURE set_unique_title_slug();
