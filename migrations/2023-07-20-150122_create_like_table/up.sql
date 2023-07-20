-- Your SQL goes here
CREATE TABLE "like" (
	id SERIAL PRIMARY KEY,
	user_id SERIAL REFERENCES user_table(id),
	post_id SERIAL REFERENCES post(id),
	reaction boolean NOT NULL
);