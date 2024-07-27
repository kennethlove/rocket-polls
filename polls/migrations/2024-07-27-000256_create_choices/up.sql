-- Your SQL goes here
CREATE TABLE choices (
  id INTEGER PRIMARY KEY,
  question_id INTEGER NOT NULL,
  choice_text VARCHAR NOT NULL,
  votes INTEGER DEFAULT 0
)
