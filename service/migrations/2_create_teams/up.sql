CREATE TABLE teams (
  id TEXT NOT NULL PRIMARY KEY,
  event TEXT NOT NULL,
  org TEXT NOT NULL,
  name TEXT NOT NULL,
  group TEXT REFERENCES groups(id),
  contact_name VARCHAR(50),
  contact_phone VARCHAR(50)
);
