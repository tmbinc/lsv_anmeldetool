DROP TABLE results;
CREATE TABLE results (
  event TEXT NOT NULL REFERENCES event(id),
  group_id TEXT NOT NULL REFERENCES groups(id),
  round INTEGER NOT NULL,
  team TEXT REFERENCES teams(id),
  rank REAL,
  points_team REAL,
  points_team_lost REAL,
  points_player REAL,
  points_player_lost REAL,
  points_win REAL,
  points_draw REAL,
  points_lost REAL,
  tie REAL,
  PRIMARY KEY(event, round, team)
);
