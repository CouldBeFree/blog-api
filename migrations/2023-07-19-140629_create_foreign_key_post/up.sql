-- Your SQL goes here
ALTER TABLE post ADD COLUMN user_id INTEGER;
ALTER TABLE post
ADD CONSTRAINT fk_post_user_id
FOREIGN KEY (user_id) REFERENCES user_table (id);