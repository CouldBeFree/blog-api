-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE post (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    content VARCHAR,
    date timestamp NOT NULL DEFAULT NOW()
);