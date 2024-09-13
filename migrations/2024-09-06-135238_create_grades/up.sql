CREATE TABLE grades (
  teacher_id INT NOT NULL,
  assignment_id INT NOT NULL,
  reduction_id INT NOT NULL,
  FOREIGN KEY (teacher_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (assignment_id) REFERENCES ASSIGNMENTS(id) ON DELETE CASCADE,
  FOREIGN KEY (reduction_id) REFERENCES REDUCTIONS(id) ON DELETE CASCADE,
  PRIMARY KEY (teacher_id, assignment_id, reduction_id)
)
