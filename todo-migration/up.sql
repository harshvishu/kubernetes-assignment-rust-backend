-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE
);
