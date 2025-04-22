use super::status::Status;
use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

pub type TaskId = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

impl Task {
    pub fn new(id: TaskId, description: String) -> Self {
        let t = Utc::now();
        Task { id, description, status: Status::ToDo, created_at: t, updated_at: t }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn create_task() {
        let task = Task::new(1, "some description".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "some description".to_string());
        assert_eq!(task.status, Status::ToDo);
        assert_eq!(task.created_at, task.updated_at);
    }

    #[test]
    fn change_description() {
        let mut task = Task::new(1, "some description".to_string());
        let previous_updated_at = task.updated_at;

        thread::sleep(Duration::from_millis(100));
        task.set_description("new description".to_string());

        assert_eq!(task.id, 1);
        assert_eq!(task.description, "new description".to_string());
        assert_eq!(task.status, Status::ToDo);
        assert_ne!(task.created_at, task.updated_at);
        assert_ne!(task.updated_at, previous_updated_at);
    }

    #[test]
    fn change_status() {
        let mut task = Task::new(1, "some description".to_string());

        for s in vec![Status::InProgress, Status::Done, Status::ToDo] {
            let previous_updated_at = task.updated_at;

            thread::sleep(Duration::from_millis(100));
            task.set_status(s.clone());

            assert_eq!(task.id, 1);
            assert_eq!(task.description, "some description".to_string());
            assert_eq!(task.status, s);
            assert_ne!(task.created_at, task.updated_at);
            assert_ne!(task.updated_at, previous_updated_at);
        }
    }
}