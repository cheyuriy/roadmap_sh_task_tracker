# Task Tracker CLI

This is a simple command-line interface (CLI) application for managing tasks. It allows you to add, update, delete, and list tasks, as well as mark them as "in progress" or "done." Tasks are stored in a JSON file for persistence.

This project is part of the [roadmap.sh backend projects](https://roadmap.sh/backend/projects).

## Features

- Add tasks with descriptions.
- Update task descriptions.
- Delete tasks.
- Mark tasks as "in progress" or "done."
- List tasks by status or all tasks.
- Peristing data in JSON-file

## Project Structure
```
. 
├── Cargo.toml # Rust project configuration 
├── src/ 
│ ├── lib.rs # Library entry point 
│ ├── main.rs # CLI entry point 
│ └── models/ # Core models and logic 
│   ├── command.rs # Command enum for CLI actions 
│   ├── mod.rs # Module declarations 
│   ├── status.rs # Task status definitions 
│   ├── store.rs # Store for managing tasks 
│   └── task.rs # Task model 
└── README.md 
```

## Usage

### Adding a Task
```bash
task-cli add "Buy groceries"
# Output: Added new task with ID 1
```

### Updating a Task
```bash
task-cli update 1 "Buy groceries and cook dinner"
```

### Deleting a Task
```bash
task-cli delete 1
```

### Marking a Task as In Progress
```bash
task-cli mark-in-progress 1
```

### Marking a Task as Done
```bash
task-cli mark-done 1
```

### Listing All Tasks
```bash
task-cli list
```

### Listing Tasks by Status
```bash
task-cli list done
task-cli list todo
task-cli list in-progress
```

### Persisted data
By default this tool saves data in JSON-file in `data/data.json` (relative to current working directory).

## Getting Started
1. Clone the repository:
```bash
git clone <repository-url>
cd roadmap_sh_task_tracker
```

2. Build the project:
```bash
cargo build
```

3. Run the CLI:
```bash
cargo run -- <command>
```

