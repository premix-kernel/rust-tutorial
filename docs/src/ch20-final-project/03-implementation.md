# Implementation

## main.rs

```rust
use std::env;
use todo_cli::{run, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let title = args[2..].join(" ");
            Command::Add(title)
        }
        Some("list") => Command::List,
        Some("complete") => {
            let id: u32 = args[2].parse().expect("Invalid ID");
            Command::Complete(id)
        }
        Some("remove") => {
            let id: u32 = args[2].parse().expect("Invalid ID");
            Command::Remove(id)
        }
        _ => Command::Help,
    };

    if let Err(e) = run(command) {
        eprintln!("Error: {}", e);
    }
}
```

---

## lib.rs

```rust
mod todo;
mod storage;

pub use todo::Todo;

pub enum Command {
    Add(String),
    List,
    Complete(u32),
    Remove(u32),
    Help,
}

pub fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let mut todos = storage::load()?;

    match command {
        Command::Add(title) => {
            let id = todos.len() as u32 + 1;
            let todo = Todo::new(id, title.clone());
            todos.push(todo);
            storage::save(&todos)?;
            println!("✅ Added: {}", title);
        }
        Command::List => {
            for todo in &todos {
                let status = if todo.completed { "x" } else { " " };
                println!("{}. [{}] {}", todo.id, status, todo.title);
            }
        }
        Command::Complete(id) => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = true;
                storage::save(&todos)?;
                println!("✅ Completed: {}", todo.title);
            }
        }
        Command::Remove(id) => {
            todos.retain(|t| t.id != id);
            storage::save(&todos)?;
            println!("✅ Removed task {}", id);
        }
        Command::Help => {
            println!("Usage: todo <command> [args]");
            println!("Commands: add, list, complete, remove");
        }
    }

    Ok(())
}
```

---

## storage.rs

```rust
use crate::Todo;
use std::fs;

const FILE_PATH: &str = "todos.json";

pub fn load() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    if !std::path::Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(FILE_PATH)?;
    let todos: Vec<Todo> = serde_json::from_str(&content)?;
    Ok(todos)
}

pub fn save(todos: &[Todo]) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(todos)?;
    fs::write(FILE_PATH, content)?;
    Ok(())
}
```

👉 ต่อไป: [สรุป](./04-review.md)
