-- Your SQL goes here
ALTER TABLE tasks
ADD COLUMN user_id INT REFERENCES users(id),
ADD COLUMN project_id INT NOT NULL REFERENCES projects(id);
