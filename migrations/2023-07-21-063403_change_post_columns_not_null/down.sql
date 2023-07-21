-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE post ALTER COLUMN title DROP NOT NULL;
ALTER TABLE post ALTER COLUMN content DROP NOT NULL;
ALTER TABLE post ALTER COLUMN user_id DROP NOT NULL;