-- This file should undo anything in `up.sql`
ALTER TABLE questions ADD COLUMN pub_date DATE;
ALTER TABLE questions DROP COLUMN published;
