CREATE TABLE users (
  id serial PRIMARY KEY,
  name varchar(255) NOT NULL,
  phone varchar(20) NOT NULL,

  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT phone_unique UNIQUE (phone)
);

ALTER TABLE posts 
  ADD user_id int;

ALTER TABLE posts
  ADD CONSTRAINT FK_post_person 
	FOREIGN KEY (user_id) REFERENCES users(id);
