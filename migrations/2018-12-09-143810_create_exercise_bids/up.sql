DROP TABLE exercises;
CREATE TABLE exercises (
  id INTEGER PRIMARY KEY NOT NULL,
  deal_id INTEGER NOT NULL,
  bids TEXT NOT NULL,
  FOREIGN KEY(deal_id) REFERENCES deals(id)
);

CREATE TABLE exercise_bids (
  id INTEGER PRIMARY KEY NOT NULL,
  exercise_id INTEGER NOT NULL,
  bid TEXT NOT NULL,
  FOREIGN KEY(exercise_id) REFERENCES exercises(id)
);
