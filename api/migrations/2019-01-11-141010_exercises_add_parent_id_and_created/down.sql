ALTER TABLE exercises RENAME TO exercises_tmp;

CREATE TABLE exercises (
  id INTEGER PRIMARY KEY NOT NULL,
  deal_id INTEGER NOT NULL,
  bids TEXT NOT NULL,
  FOREIGN KEY(deal_id) REFERENCES deals(id)
);

INSERT INTO exercises (id, deal_id, bids)
  SELECT id, deal_id, bids
  FROM exercises_tmp;

DROP TABLE exercises_tmp;
