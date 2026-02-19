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
