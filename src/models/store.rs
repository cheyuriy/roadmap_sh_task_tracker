use super::task::{Task, TaskId};
use super::status::Status;
use std::fs;
use std::io::Write;

#[derive(Debug)]
pub struct Store {
    tasks: Vec<Task>,
    max_task_id: TaskId,
    path: String
}

impl Store {
    pub fn new() -> Self {
        let path = "data/data.json";
        if let Ok(is_exists) = fs::exists(path) {
            if is_exists {
                let data = fs::read_to_string(path).expect("Unable to read file");
                let tasks: Vec<Task> = serde_json::from_str(&data).expect("Unable to parse JSON");
                let max_task_id = tasks.iter().map(|i| i.id()).max().unwrap_or(0);
                Store { tasks, max_task_id, path: path.to_string() }
            } else {
                let empty_tasks: Vec<Task> = vec![];
                let s = Store {tasks: empty_tasks, max_task_id: 0, path: path.to_string()};
                s.persist();
                s
            }
        } else {
            panic!("Can't check existence of file `data.json`");
        }
    }

    pub fn persist(&self) {
        let json = serde_json::to_string_pretty(&self.tasks).expect("Unable to write JSON");
        let mut file = fs::File::create(&self.path).expect("Unable to create file");
        file.write_all(json.as_bytes()).expect("Unable to write file");
    }
    
    pub fn add_task(&mut self, description: String) -> TaskId {
        let id = self.max_task_id + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
        self.max_task_id = id;
        self.persist();
        id
    }

    pub fn update_task(&mut self, id: TaskId, description: String) {
        if let Some(item) = self.tasks.iter_mut().find(|i| i.id() == id) {
            item.set_description(description);
        } else {
            println!("Didn't find a task with ID {id}");
        };
        self.persist();
    }

    pub fn delete_task(&mut self, id: TaskId) {
        if let Some(pos) = self.tasks.iter().position(|i| i.id() == id) {
            self.tasks.remove(pos);
        } else {
            println!("Didn't find a task with ID {id}");
        };
        self.persist();
    }

    pub fn mark_task(&mut self, id: TaskId, status: Status) {
        if let Some(item) = self.tasks.iter_mut().find(|i| i.id() == id) {
            item.set_status(status);
        } else {
            println!("Didn't find a task with ID {id}");
        };
        self.persist();
    }

    pub fn list_tasks(&self, status: Option<Status>) -> Vec<&Task> {
        if let Some(status) = status {
            self.tasks.iter().filter(|i| *i.status() == status).collect()
        } else {
            self.tasks.iter().collect()
        }
    }
}