DROP TABLE exercise_bids;
CREATE TABLE exercise_bids (
  id INTEGER PRIMARY KEY NOT NULL,
  exercise_id INTEGER NOT NULL,
  bid TEXT NOT NULL,
  FOREIGN KEY(exercise_id) REFERENCES exercises(id)
);
