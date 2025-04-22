use roadmap_sh_task_tracker::{Command, TaskId, Store, Status};
use std::env;

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
                println!("ID {0}: {1}. Status: {2}", item.id(), item.description(), item.status());
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
