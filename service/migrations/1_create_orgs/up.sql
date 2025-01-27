CREATE TABLE orgs (
  id TEXT NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  public BOOLEAN NOT NULL DEFAULT 0,
  contact_email VARCHAR(50),
  contact_name VARCHAR(50),
  contact_phone VARCHAR(50),
  confirmed_email BOOLEAN NOT NULL DEFAULT 0,
  last_update DATETIME NOT NULL
);
