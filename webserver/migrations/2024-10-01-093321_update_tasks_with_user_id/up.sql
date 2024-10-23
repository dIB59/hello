-- Your SQL goes here
ALTER TABLE tasks
ADD COLUMN user_id INT REFERENCES users(id),
ADD COLUMN project_id INT NOT NULL REFERENCES projects(id);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
