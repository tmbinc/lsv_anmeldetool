CREATE TABLE templates_old (
  event TEXT NOT NULL REFERENCES event(id),
  template_type TEXT NOT NULL,
  template_variant TEXT NOT NULL,
  content TEXT NOT NULL,
  PRIMARY KEY(event, template_type)
);

INSERT INTO templates_old SELECT * FROM templates;
DROP TABLE templates;
ALTER TABLE templates_old RENAME TO templates;
