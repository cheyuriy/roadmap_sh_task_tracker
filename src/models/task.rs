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