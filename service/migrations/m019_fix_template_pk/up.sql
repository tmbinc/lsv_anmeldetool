CREATE TABLE templates_new (
  event TEXT NOT NULL REFERENCES event(id),
  template_type TEXT NOT NULL,
  template_variant TEXT NOT NULL,
  content TEXT NOT NULL,
  PRIMARY KEY(event, template_type, template_variant)
);

INSERT INTO templates_new SELECT * FROM templates;
DROP TABLE templates;
ALTER TABLE templates_new RENAME TO templates;
