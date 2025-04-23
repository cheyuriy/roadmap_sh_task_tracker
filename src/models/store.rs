use super::task::{Task, TaskId};
use super::status::Status;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Store {
    tasks: Vec<Task>,
    max_task_id: TaskId,
    path: String
}

impl Store {
    pub fn new(file_path: Option<&str>) -> Self {
        let path = if let Some(p) = file_path  {
            p 
        } else {
            "data/data.json"            
        };
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

        let path = Path::new(&self.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Unable to create directory");
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn create_store() {
        let temp_file = "tmp1.json";
        let store = Store::new(Some(temp_file));
        assert_eq!(store.tasks.len(), 0);
        assert_eq!(store.max_task_id, 0);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn add_task() {
        let temp_file = "tmp2.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_task("some description".to_string());
        assert_eq!(store.tasks.len(), 1);
        assert_eq!(store.max_task_id, id);
        assert_eq!(store.tasks[0].description(), "some description");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn update_task() {
        let temp_file = "tmp3.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_task("some description".to_string());
        store.update_task(id, "new description".to_string());
        assert_eq!(store.tasks[0].description(), "new description");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn delete_task() {
        let temp_file = "tmp4.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_task("some description".to_string());
        store.delete_task(id);
        assert_eq!(store.tasks.len(), 0);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn mark_task() {
        let temp_file = "tmp5.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_task("some description".to_string());
        store.mark_task(id, Status::Done);
        assert_eq!(store.tasks[0].status(), &Status::Done);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn list_tasks() {
        let temp_file = "tmp6.json";
        let mut store = Store::new(Some(temp_file));
        store.add_task("some description".to_string());
        store.add_task("another description".to_string());
        let tasks = store.list_tasks(None);
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].description(), "some description");
        assert_eq!(tasks[1].description(), "another description");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn list_tasks_with_status() {
        let temp_file = "tmp7.json";
        let mut store = Store::new(Some(temp_file));
        let id1 = store.add_task("some description".to_string());
        let _ = store.add_task("another description".to_string());
        store.mark_task(id1, Status::Done);
        let tasks = store.list_tasks(Some(Status::Done));
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id(), id1);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn persist_store() {
        let temp_file = "tmp8.json";
        let mut store = Store::new(Some(temp_file));
        store.add_task("some description".to_string());
        store.persist();
        let data = fs::read_to_string(&store.path).expect("Unable to read file");
        let tasks: Vec<Task> = serde_json::from_str(&data).expect("Unable to parse JSON");
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description(), "some description");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn persist_store_with_empty_tasks() {
        let temp_file = "tmp9.json";
        let store = Store::new(Some(temp_file));
        let data = fs::read_to_string(&store.path).expect("Unable to read file");
        let tasks: Vec<Task> = serde_json::from_str(&data).expect("Unable to parse JSON");
        assert_eq!(tasks.len(), 0);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }
}