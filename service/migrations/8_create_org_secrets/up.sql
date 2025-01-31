CREATE TABLE org_secrets (
  org_id TEXT NOT NULL REFERENCES orgs(id),  
  secret VARCHAR(40) NOT NULL,
  PRIMARY KEY(org_id, secret)
);
