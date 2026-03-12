CREATE TABLE templates (
  event TEXT NOT NULL REFERENCES event(id),
  template_type TEXT NOT NULL,
  template_variant TEXT NOT NULL,
  content TEXT NOT NULL,
  PRIMARY KEY(event, template_type)
);
