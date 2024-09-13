CREATE TABLE assignments (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name CITEXT NOT NULL,
  subject_id INT NOT NULL,
  deadline TIMESTAMP NOT NULL,
  FOREIGN KEY (SUBJECT_ID) REFERENCES subjects(id),
  -- this unique could indeed be a primary key
  UNIQUE (name, subject_id)
);

CREATE TABLE questions (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  content TEXT NOT NULL,
  points INT NOT NULL,
  assignment_id INT NOT NULL,
  FOREIGN KEY(ASSIGNMENT_ID) REFERENCES assignments(id) ON DELETE CASCADE,
  -- this unique could indeed be primary key
  UNIQUE (content, assignment_id)
);

CREATE TABLE reductions (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  content CITEXT NOT NULL,
  points INT NOT NULL,
  question_id INT NOT NULL,
  FOREIGN KEY(QUESTION_ID) REFERENCES questions(id) ON DELETE CASCADE,
  -- this unique could also be a primary key constraint
  UNIQUE (content, question_id)
);
