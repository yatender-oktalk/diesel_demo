-- This file should undo anything in `up.sql`
DROP TABLE users;

ALTER TABLE posts
DROP CONSTRAINT FK_post_person;

ALTER TABLE posts
DROP COLUMN user_id;