CREATE EXTENSION citext;

CREATE TYPE ROLE AS ENUM ('admin', 'teacher', 'student');

CREATE TABLE users (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  email CITEXT NOT NULL,
  role ROLE NOT NULL,
  UNIQUE (email, role)
)
