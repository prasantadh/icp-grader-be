create table enrols (
  student_id int not null,
  level_id int not null,
  foreign key (student_id) references users(id) on delete cascade,
  foreign key (level_id) references users(id) on delete cascade,
  PRIMARY KEY(student_id, level_id)
)
