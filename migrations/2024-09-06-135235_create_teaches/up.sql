CREATE TYPE VALID_LESSONS AS ENUM ('lecture', 'tutorial', 'workshop');

CREATE TABLE teaches (
  teacher_id INT NOT NULL,
  lesson_type VALID_LESSONS NOT NULL,
  section_id CITEXT not null,
  FOREIGN KEY(teacher_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY(section_id) REFERENCES SECTIONS(id) ON DELETE CASCADE,
  PRIMARY KEY (teacher_id, lesson_type, section_id)
);
