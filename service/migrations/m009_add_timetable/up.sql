ALTER TABLE groups ADD COLUMN num_rounds INTEGER NOT NULL DEFAULT 7;
CREATE TABLE timetable (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  row_index INTEGER NOT NULL,
  name TEXT NOT NULL,
  expected_time DATETIME NOT NULL,
  last_update DATETIME NOT NULL,
  state TEXT NOT NULL DEFAULT "",

  PRIMARY KEY(event, group_id, row_index)
);
