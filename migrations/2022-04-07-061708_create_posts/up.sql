-- Your SQL goes here
CREATE TABLE posts (
  id serial PRIMARY KEY,
  title varchar(255) NOT NULL,
  body text NOT NULL,
  published boolean DEFAULT true,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
)