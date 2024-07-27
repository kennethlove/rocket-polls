-- Your SQL goes here
ALTER TABLE questions ADD COLUMN published BOOLEAN;
ALTER TABLE questions DROP COLUMN pub_date;
