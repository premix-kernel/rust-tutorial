# การออกแบบ

ออกแบบ CLI Todo App ให้รองรับการขยาย

## Requirements

### Functional Requirements

1. เพิ่ม todo ใหม่ได้
2. แสดงรายการ todos ทั้งหมด
3. ทำเครื่องหมาย complete ได้
4. ลบ todo ได้
5. บันทึกลงไฟล์ (persistent)

### Non-functional Requirements

1. ใช้งานง่าย (simple CLI)
2. Error handling ที่ดี
3. Code ที่ test ได้
4. แยก concerns ชัดเจน

---

## โครงสร้างโปรเจกต์

```text
todo-cli/
├── Cargo.toml
├── src/
│   ├── main.rs      ← Entry point, CLI parsing
│   ├── lib.rs       ← Application logic
│   ├── todo.rs      ← Todo struct
│   └── storage.rs   ← File I/O
├── tests/
│   └── integration_test.rs
└── todos.json       ← Data storage
```

---

## Architecture Diagram

```text
+---------------------------------------------+
|                   main.rs                   |
|   - Parse CLI arguments                     |
|   - Create Command enum                     |
|   - Call lib::run()                         |
+----------------------+----------------------+
                       |
                       v
+---------------------------------------------+
|                   lib.rs                    |
|   - Application logic                       |
|   - Handle Commands                         |
|   - Orchestrate todo + storage              |
+----------------------+----------------------+
                       |
         +-------------+-------------+
         v                           v
+-----------------+         +-----------------+
|    todo.rs      |         |   storage.rs    |
|   - Todo struct |         |   - load()      |
|   - new()       |         |   - save()      |
|   - toggle()    |         |   - JSON I/O    |
+-----------------+         +-----------------+
```

---

## Data Structures

### Todo Struct

```rust,ignore
// src/todo.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
        if self.completed {
            self.completed_at = Some(Utc::now());
        } else {
            self.completed_at = None;
        }
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.completed { "✓" } else { " " };
        write!(f, "[{}] {}. {}", status, self.id, self.title)
    }
}
```

---

## Command Enum

```rust,ignore
// src/lib.rs
#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String),
    List,
    Complete(u32),
    Uncomplete(u32),
    Remove(u32),
    Clear,
    Help,
}

impl Command {
    pub fn from_args(args: &[String]) -> Self {
        match args.get(1).map(|s| s.as_str()) {
            Some("add") => {
                let title = args[2..].join(" ");
                if title.is_empty() {
                    Command::Help
                } else {
                    Command::Add(title)
                }
            }
            Some("list") | Some("ls") => Command::List,
            Some("complete") | Some("done") => {
                args.get(2)
                    .and_then(|s| s.parse().ok())
                    .map(Command::Complete)
                    .unwrap_or(Command::Help)
            }
            Some("uncomplete") | Some("undo") => {
                args.get(2)
                    .and_then(|s| s.parse().ok())
                    .map(Command::Uncomplete)
                    .unwrap_or(Command::Help)
            }
            Some("remove") | Some("rm") => {
                args.get(2)
                    .and_then(|s| s.parse().ok())
                    .map(Command::Remove)
                    .unwrap_or(Command::Help)
            }
            Some("clear") => Command::Clear,
            _ => Command::Help,
        }
    }
}
```

---

## Modules Responsibility

| Module       | Responsibility           | Dependencies     |
| ------------ | ------------------------ | ---------------- |
| `main.rs`    | CLI parsing, entry point | lib              |
| `lib.rs`     | Application logic        | todo, storage    |
| `todo.rs`    | Todo data model          | serde, chrono    |
| `storage.rs` | File I/O                 | serde_json, todo |

---

## Error Handling Strategy

```rust,ignore
// src/lib.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Todo with id {0} not found")]
    NotFound(u32),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Invalid command")]
    InvalidCommand,
}

// src/storage.rs
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}
```

---

## Dependencies

```toml,ignore
# Cargo.toml
[package]
name = "todo-cli"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
```

---

## CLI Design

```bash
# Add todo
$ todo add Buy groceries
✅ Added: Buy groceries

# List todos
$ todo list
[✓] 1. Buy groceries
[ ] 2. Read Rust book
[ ] 3. Exercise

# Complete todo
$ todo complete 2
✅ Completed: Read Rust book

# Remove todo
$ todo remove 1
✅ Removed: Buy groceries

# Help
$ todo help
Usage: todo <command> [args]

Commands:
  add <title>      Add a new todo
  list             List all todos
  complete <id>    Mark todo as complete
  remove <id>      Remove a todo
  clear            Remove all todos
  help             Show this help
```

---

## ลองทำดู! 🎯

1. ออกแบบ struct เพิ่มเติม (priority, tags)
2. วาด sequence diagram สำหรับ "add todo"
3. ออกแบบ error types

---

## สรุป

| Component  | Role                          |
| ---------- | ----------------------------- |
| main.rs    | Parse args → Command → run()  |
| lib.rs     | Handle command → update state |
| todo.rs    | Data model                    |
| storage.rs | Persistence layer             |

### Design Principles Used

- **Single Responsibility**: แต่ละ module ทำหน้าที่เดียว
- **Dependency Injection**: Storage trait สำหรับ testing
- **Error Handling**: Custom error types พร้อม context
- **Separation of Concerns**: UI/Logic/Data แยกกัน

👉 ต่อไป: [Implementation](./03-implementation.md)
