-- Your SQL goes here
CREATE TABLE timeslots (
  id SERIAL PRIMARY KEY,
  trainer_id INTEGER NOT NULL,
  start TIMESTAMP NOT NULL,
  duration INTEGER NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL,
  user_id INTEGER NULL
);
