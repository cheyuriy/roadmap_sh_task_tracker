use std::env;
use std::fmt::Display;
use std::fs;
use std::io::Write;
use chrono::prelude::{DateTime, Utc};
use serde::{Serialize, Deserialize};

type TaskId = usize;

#[derive(Debug)]
enum Command {
    Add(String),
    Update(TaskId, String),
    Delete(TaskId),
    MarkInProgress(TaskId),
    MarkDone(TaskId),
    List(Option<Status>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Status {
    Done,
    ToDo,
    InProgress
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Done => write!(f, "Done"),
            Self::ToDo => write!(f, "ToDo"),
            Self::InProgress => write!(f, "In Progress")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: TaskId,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

impl Task {
    fn new(id: TaskId, description: String) -> Self {
        let t = Utc::now();
        Task { id, description, status: Status::ToDo, created_at: t, updated_at: t }
    }
}

#[derive(Debug)]
struct Store {
    tasks: Vec<Task>,
    max_task_id: TaskId,
    path: String
}

impl Store {
    fn new() -> Self {
        let path = "data.json";
        if let Ok(is_exists) = fs::exists(path) {
            if is_exists {
                let data = fs::read_to_string(path).expect("Unable to read file");
                let tasks: Vec<Task> = serde_json::from_str(&data).expect("Unable to parse JSON");
                let max_task_id = tasks.iter().map(|i| i.id).max().unwrap_or(0);
                Store { tasks, max_task_id, path: "data.json".to_string() }
            } else {
                let empty_tasks: Vec<Task> = vec![];
                let s = Store {tasks: empty_tasks, max_task_id: 0, path: "data.json".to_string()};
                s.persist();
                s
            }
        } else {
            panic!("Can't check existence of file `data.json`");
        }
    }

    fn persist(&self) {
        let json = serde_json::to_string_pretty(&self.tasks).expect("Unable to write JSON");
        let mut file = fs::File::create(&self.path).expect("Unable to create file");
        file.write_all(json.as_bytes()).expect("Unable to write file");
    }
    
    fn add_task(&mut self, description: String) -> TaskId {
        let id = self.max_task_id + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
        self.max_task_id = id;
        self.persist();
        id
    }

    fn update_task(&mut self, id: TaskId, description: String) {
        if let Some(item) = self.tasks.iter_mut().find(|i| i.id == id) {
            item.description = description;
            item.updated_at = Utc::now();
        } else {
            println!("Didn't find a task with ID {id}");
        };
        self.persist();
    }

    fn delete_task(&mut self, id: TaskId) {
        if let Some(pos) = self.tasks.iter().position(|i| i.id == id) {
            self.tasks.remove(pos);
        } else {
            println!("Didn't find a task with ID {id}");
        };
        self.persist();
    }

    fn mark_task(&mut self, id: TaskId, status: Status) {
        if let Some(item) = self.tasks.iter_mut().find(|i| i.id == id) {
            item.status = status;
            item.updated_at = Utc::now();
        } else {
            println!("Didn't find a task with ID {id}");
        };
        self.persist();
    }

    fn list_tasks(&self, status: Option<Status>) -> Vec<&Task> {
        if let Some(status) = status {
            self.tasks.iter().filter(|i| i.status == status).collect()
        } else {
            self.tasks.iter().collect()
        }
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args);
    let mut store = Store::new();
    match command {
        Command::Add(description) => {
            let new_id = store.add_task(description);
            println!("Added new task with ID {new_id}");
        },
        Command::Delete(id) => {
            store.delete_task(id);
        },
        Command::Update(id, description ) => {
            store.update_task(id, description);
        },
        Command::MarkInProgress(id) => {
            store.mark_task(id, Status::InProgress);
        },
        Command::MarkDone(id) => {
            store.mark_task(id, Status::Done);
        },
        Command::List(status) => {
            let result = store.list_tasks(status);
            for item in result  {
                println!("ID {0}: {1}. Status: {2}", item.id, item.description, item.status);
            };
        }
    }
}

fn parse_args(args: &Vec<String>) -> Command {
    let command_arg = args.get(1);
    let command = if let Some(val) = command_arg {
        match val.as_str() {
            "add" => {
                let description = args.get(2).expect("ADD command expects a task's description as the second argument").into();
                Command::Add(description)
            },
            "update" => {
                let id = args
                    .get(2)
                    .expect("UPDATE command expects a task's ID as the second argument")
                    .parse::<TaskId>()
                    .expect("UPDATE command expects task's ID to be an integer number");
                let description = args.get(3).expect("UPDATE command expects a task's new description as the third argument").into();
                Command::Update(id, description)
            },
            "delete" => {
                let id = args
                    .get(2)
                    .expect("UPDATE command expects a task's ID as the second argument")
                    .parse::<TaskId>()
                    .expect("UPDATE command expects task's ID to be an integer number");
                Command::Delete(id)
            },
            "mark-in-progress" => {
                let id = args
                    .get(2)
                    .expect("UPDATE command expects a task's ID as the second argument")
                    .parse::<TaskId>()
                    .expect("UPDATE command expects task's ID to be an integer number");
                Command::MarkInProgress(id)
            },
            "mark-done" => {
                let id = args
                    .get(2)
                    .expect("UPDATE command expects a task's ID as the second argument")
                    .parse::<TaskId>()
                    .expect("UPDATE command expects task's ID to be an integer number");
                Command::MarkDone(id)
            },
            "list" => {
                let list_filter = if let Some(val) = args.get(2) {
                    match val.as_str() {
                        "done" => Some(Status::Done),
                        "todo" => Some(Status::ToDo),
                        "in-progress" => Some(Status::InProgress),
                        _ => panic!("Unknown filter for LIST command")
                    }
                } else {
                    None
                };
                Command::List(list_filter)
            }
            _ => {
                panic!("Unknown command");
            }
        }
    } else {
        panic!("Should be run with arguments");
    };
    return command;
}
