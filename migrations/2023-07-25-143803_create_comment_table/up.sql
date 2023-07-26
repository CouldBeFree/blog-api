-- Your SQL goes here
CREATE TABLE "comment" (
	id SERIAL PRIMARY KEY,
	user_id SERIAL REFERENCES user_table(id),
	post_id SERIAL REFERENCES post(id),
	content VARCHAR NOT NULL,
    date timestamp NOT NULL DEFAULT NOW()
);