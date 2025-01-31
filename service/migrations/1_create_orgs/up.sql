CREATE TABLE orgs (
  id TEXT NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  name_additional VARCHAR,
  genus CHAR,
  public BOOLEAN NOT NULL DEFAULT 0,
  contact_email VARCHAR(50),
  contact_email_pending VARCHAR(50),
  contact_name VARCHAR(50),
  contact_phone VARCHAR(50),
  last_update DATETIME NOT NULL
);
