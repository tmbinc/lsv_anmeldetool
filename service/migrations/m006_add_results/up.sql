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
