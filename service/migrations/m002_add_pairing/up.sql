CREATE TABLE pairings (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  round INTEGER NOT NULL,
  table_num INTEGER NOT NULL,
  team_home TEXT REFERENCES teams(id),
  team_guest TEXT REFERENCES teams(id),
  points_home INTEGER,
  points_guest INTEGER,
  result TEXT,
  PRIMARY KEY(event, round, team_home, team_guest)
);

CREATE TABLE rooms (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  table_num_low INTEGER NOT NULL,
  table_num_high INTEGER NOT NULL,
  room TEXT NOT NULL,
  PRIMARY KEY(event, group_id, table_num_low, table_num_high)
);
