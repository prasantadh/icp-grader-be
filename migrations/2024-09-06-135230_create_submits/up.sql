CREATE TABLE submits (
  url CITEXT NOT NULL,
  token CITEXT,
  assignment_id INT NOT NULL,
  student_id INT NOT NULL,
  FOREIGN KEY(assignment_id) REFERENCES assignments(id) ON DELETE CASCADE,
  FOREIGN KEY(student_id) REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY(student_id, assignment_id)
  -- FIXME: should we do a unique contraint on url, token
)
