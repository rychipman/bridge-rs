DROP TABLE hands;
CREATE TABLE deals (
  id INTEGER PRIMARY KEY NOT NULL,
  dealer TEXT NOT NULL,
  vulnerable TEXT NOT NULL,
  hand TEXT NOT NULL
);
