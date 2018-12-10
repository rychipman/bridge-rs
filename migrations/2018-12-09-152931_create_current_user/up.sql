CREATE TABLE current_user (
  id INTEGER PRIMARY KEY NOT NULL CHECK (id = 1),
  user_id INTEGER NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
);
