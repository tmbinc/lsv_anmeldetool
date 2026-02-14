DROP TABLE results;
CREATE TABLE results (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  round INTEGER NOT NULL,
  team TEXT REFERENCES teams(id),
  rank REAL,
  points_team REAL,
  points_player REAL,
  tie REAL,
  PRIMARY KEY(event, round, team)
);

DROP TABLE pairings;
CREATE TABLE pairings (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  round INTEGER NOT NULL,
  table_num INTEGER NOT NULL,
  team_home TEXT REFERENCES teams(id),
  team_guest TEXT REFERENCES teams(id),
  points_home REAL,
  points_guest REAL,
  result TEXT,
  PRIMARY KEY(event, round, team_home, team_guest)
);