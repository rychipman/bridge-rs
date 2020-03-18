DROP TABLE comments;
CREATE TABLE comments (
  id INTEGER PRIMARY KEY NOT NULL,
  user_id INTEGER NOT NULL,
  exercise_bid_id INTEGER NOT NULL,
  text TEXT NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(exercise_bid_id) REFERENCES exercise_bids(id)
);
