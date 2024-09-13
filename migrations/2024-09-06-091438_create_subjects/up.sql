CREATE TABLE SUBJECTS (
  id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  -- MAY BE EVENTUALLY MAKE THIS SUCH THAT ONE OF THE 
  -- THREE FIELDS AMONG YEAR, LEVEL, AND SECTIONS IS NOT EMPTY
  -- This would make the unique constraint very difficult
  -- FOR NOW GO WITH LEVELS
  name CITEXT NOT NULL,
  level_id CITEXT NOT NULL,
  UNIQUE (level_id, name),
  FOREIGN KEY (LEVEL_ID) REFERENCES LEVELS(ID) ON DELETE CASCADE
  -- on the application side, only an admin can create a subject
)
