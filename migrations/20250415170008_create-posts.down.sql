-- Add down migration script here
DROP VIEW post_view;
DROP TABLE posts;
DROP EXTENSION "uuid-ossp";