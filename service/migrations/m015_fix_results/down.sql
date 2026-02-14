DROP TABLE results;
CREATE TABLE results (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  round INTEGER NOT NULL,
  team TEXT REFERENCES teams(id),
  rank INTEGER,
  points_team INTEGER,
  points_player INTEGER,
  tie INTEGER,
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
  points_home INTEGER,
  points_guest INTEGER,
  result TEXT,
  PRIMARY KEY(event, round, team_home, team_guest)
);