ALTER TABLE events ADD COLUMN registration_active BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE events ADD COLUMN registration_start_date DATETIME;
