CREATE TABLE years (
  id INTEGER PRIMARY KEY CHECK (id > 2000 AND id < 3000)
);

CREATE TABLE levels (
  id CITEXT PRIMARY KEY,
  year_id INT NOT NULL,
  FOREIGN KEY(year_id) REFERENCES years(id) ON DELETE CASCADE,
  UNIQUE (id, year_id)
);

CREATE TABLE sections (
  id CITEXT PRIMARY KEY,
  level_id CITEXT NOT NULL,
  FOREIGN KEY(level_id) REFERENCES levels(id) ON DELETE CASCADE,
  UNIQUE(id, level_id)
);


