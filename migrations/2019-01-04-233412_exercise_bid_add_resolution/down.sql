ALTER TABLE exercise_bids RENAME TO exercise_bids_old;
CREATE TABLE exercise_bids (
  id INTEGER PRIMARY KEY NOT NULL,
  exercise_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  bid TEXT NOT NULL,
  FOREIGN KEY(exercise_id) REFERENCES exercises(id),
  FOREIGN KEY(user_id) REFERENCES users(id)
);

INSERT INTO exercise_bids
  SELECT
    id, exercise_id, user_id, bid
  FROM exercise_bids_old;

DROP TABLE exercise_bids_old;
