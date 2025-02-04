CREATE TABLE questionnaire (
  id TEXT NOT NULL PRIMARY KEY,
  event_id TEXT REFERENCES events(id),
  org_id TEXT REFERENCES orgs(id),
  question_text TEXT NOT NULL DEFAULT "",
  question_type TEXT NOT NULL DEFAULT "",
  question_data TEXT NOT NULL DEFAULT ""
);
CREATE TABLE questionnaire_answers (
  question_id TEXT NOT NULL REFERENCES questionnaire(id),
  event_id TEXT NOT NULL REFERENCES events(id),
  org_id TEXT NOT NULL REFERENCES orgs(id),
  question_answer TEXT NOT NULL,
  PRIMARY KEY(question_id, event_id, org_id)
);
