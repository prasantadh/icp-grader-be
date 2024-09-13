drop table if exists grades, submits, teaches, reductions, questions, assignments, subjects, groups, levels, years, users, levels;

drop type if exists valid_role;
CREATE TYPE valid_role AS ENUM ('admin', 'teacher', 'student');

create table users (
  id bigserial primary key,
  role valid_role,
  email varchar(255) not null
);


create table years (
  id int primary key
);

create table levels (
  id varchar(8) primary key,
  year_id int not null,
  FOREIGN KEY(year_id) REFERENCES years(id) ON DELETE CASCADE
);

create table groups (
  id varchar(8) primary key,
  level_id varchar(8) not null,
  FOREIGN KEY(level_id) REFERENCES levels(id)
);

create table subjects (
  id bigserial primary key,
  name varchar(255) not null,
  level_id varchar(8) not null,
  FOREIGN KEY(level_id) REFERENCES levels(id)
);

create table assignments (
  id bigserial primary key,
  name varchar(255) not null,
  subject_id int not null,
  foreign key (subject_id) references subjects(id)
);

create table questions (
  id bigserial primary key,
  content varchar(2048) not null,
  assignment_id int not null,
  foreign key(assignment_id) references assignments(id) on delete cascade
);

create table reductions (
  id bigserial primary key,
  content varchar(2048) not null,
  question_id int not null,
  foreign key(question_id) references questions(id) on delete cascade
);

drop type if exists valid_lesson;
create type valid_lesson as enum ('lecture', 'tutorial', 'workshop');

create table teaches (
  id bigserial primary key,
  teacher_id int not null,
  lesson valid_lesson,
  subject_id int not null,
  -- FIXME: see if we can check users(id) belongs to a certain role
  foreign key(teacher_id) references users(id) on delete cascade,
  foreign key(subject_id) references subjects(id) on delete cascade
);
-- also need a Takes table that assigns a student to a section
-- over the years a student can be a part of multple levels and sections

create table submits (
  id bigserial primary key,
  student_id int not null,
  assignment_id int not null,
  url varchar(1024) not null,
  token varchar(1024),
  foreign key(student_id) references users(id) on delete cascade,
  foreign key(assignment_id) references assignments(id) on delete cascade
);

create table grades(
  id bigserial primary key,
  teacher_id int not null,
  submission_id int not null,
  foreign key(teacher_id) references users(id) on delete cascade,
  foreign key(submission_id) references submits(id) on delete cascade
);

-- FIXME: there are multiple unique constraints to check for
-- as well as many other value constrainsts but this is looking
-- like a good place to get started
