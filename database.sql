DROP TABLE if EXISTS tasks;

CREATE TABLE tasks (
    id serial PRIMARY KEY,
    task json NOT NULL
);