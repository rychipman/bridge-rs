ALTER TABLE users RENAME TO users_tmp;

CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  email TEXT UNIQUE NOT NULL,
  pw_hash TEXT NOT NULL
);

INSERT INTO users (id, email, pw_hash)
  SELECT id, email, pw_hash
  FROM users_tmp;

DROP TABLE users_tmp;
