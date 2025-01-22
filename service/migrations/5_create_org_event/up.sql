CREATE TABLE org_event (
  event_id TEXT NOT NULL REFERENCES events(id),
  org_id TEXT NOT NULL REFERENCES orgs(id),
  state TEXT NOT NULL,
  PRIMARY KEY(event_id, org_id)
);
