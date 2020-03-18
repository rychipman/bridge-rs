DROP TABLE exercise_bids;

DROP TABLE exercises;
CREATE TABLE exercises (
  id INTEGER PRIMARY KEY NOT NULL,
  deal_id INTEGER NOT NULL,
  bids TEXT NOT NULL,
  next_bid TEXT,
  FOREIGN KEY(deal_id) REFERENCES deals(id)
);
