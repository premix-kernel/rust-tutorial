# แบบฝึกหัด: บทที่ 20 - Final Project

## แบบฝึกหัดที่ 1: Project Setup

สร้าง project structure สำหรับ CLI todo app

<details>
<summary>ดูเฉลย</summary>

```bash
cargo new todo_app
cd todo_app
cargo add clap --features derive
cargo add serde --features derive
cargo add serde_json
```

```text
todo_app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── todo.rs
│   └── storage.rs
└── data/
    └── todos.json
```

</details>

---

## แบบฝึกหัดที่ 2: Data Structure

สร้าง Todo struct

<details>
<summary>ดูเฉลย</summary>

```rust
// src/todo.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}
```

</details>

---

## แบบฝึกหัดที่ 3: Storage Layer

สร้าง module สำหรับ save/load todos

<details>
<summary>ดูเฉลย</summary>

```rust
// src/storage.rs
use crate::todo::Todo;
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "data/todos.json";

pub fn load_todos() -> Vec<Todo> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

pub fn save_todos(todos: &[Todo]) -> std::io::Result<()> {
    fs::create_dir_all("data")?;
    let json = serde_json::to_string_pretty(todos)?;
    fs::write(FILE_PATH, json)
}
```

</details>

---

## แบบฝึกหัดที่ 4: CLI Interface

ใช้ clap สร้าง command line interface

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
// src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add { title: String },
    /// List all todos
    List,
    /// Toggle todo completion
    Toggle { id: u32 },
    /// Delete a todo
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            println!("Adding: {}", title);
        }
        Commands::List => {
            println!("Listing todos...");
        }
        Commands::Toggle { id } => {
            println!("Toggling: {}", id);
        }
        Commands::Delete { id } => {
            println!("Deleting: {}", id);
        }
    }
}
```

</details>

---

## แบบฝึกหัดที่ 5: Complete Implementation

รวมทุกอย่างเข้าด้วยกัน

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
// src/main.rs
mod storage;
mod todo;

use clap::{Parser, Subcommand};
use storage::{load_todos, save_todos};
use todo::Todo;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Toggle { id: u32 },
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { title } => {
            let id = todos.len() as u32 + 1;
            todos.push(Todo::new(id, title.clone()));
            save_todos(&todos).unwrap();
            println!("✅ Added: {}", title);
        }
        Commands::List => {
            if todos.is_empty() {
                println!("No todos yet!");
            } else {
                for todo in &todos {
                    let status = if todo.completed { "✓" } else { " " };
                    println!("[{}] {} - {}", status, todo.id, todo.title);
                }
            }
        }
        Commands::Toggle { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.toggle();
                save_todos(&todos).unwrap();
                println!("Toggled: {}", todo.title);
            } else {
                println!("Todo not found");
            }
        }
        Commands::Delete { id } => {
            let len = todos.len();
            todos.retain(|t| t.id != id);
            if todos.len() < len {
                save_todos(&todos).unwrap();
                println!("Deleted todo {}", id);
            } else {
                println!("Todo not found");
            }
        }
    }
}
```

**Usage:**

```bash
cargo run -- add "Learn Rust"
cargo run -- list
cargo run -- toggle 1
cargo run -- delete 1
```

</details>

---

🎉 **ยินดีด้วย!** คุณเรียนจบครบทุกบทแล้ว!

👉 [Quiz](../quiz/README.md)
