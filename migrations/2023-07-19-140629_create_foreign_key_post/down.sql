-- This file should undo anything in `up.sql`
ALTER TABLE post
DROP CONSTRAINT fk_post_user_id;