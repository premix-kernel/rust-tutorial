# Project Overview

## Todo CLI App

สร้าง command-line todo app ที่มี features:

```text
$ todo add "Learn Rust"
✅ Added: Learn Rust

$ todo list
1. [ ] Learn Rust
2. [x] Read the book

$ todo complete 1
✅ Completed: Learn Rust

$ todo remove 1
✅ Removed: Learn Rust
```

### 🏗️ Project Architecture

```text
+-------------------------------------------------------------------+
|                    Todo CLI App Architecture                      |
+-------------------------------------------------------------------+
|                                                                   |
|   +---------------------------------------------------------------+
|   |                     main.rs (Entry Point)                     |
|   |  * Parse command-line arguments                               |
|   |  * Route to appropriate handler                               |
|   +-------------------------------+-------------------------------+
|                                   |                               |
|           +-----------------------+-----------------------+       |
|           |                       |                       |       |
|           v                       v                       v       |
|   +------------+           +------------+           +------------+|
|   |   todo.rs  |           | storage.rs |           |   cli.rs   ||
|   |  (Model)   |           |   (I/O)    |           | (Interface)||
|   |            |           |            |           |            ||
|   | struct Todo|           | load/save  |           | add, list  ||
|   | enum Status|           | JSON file  |           | complete   ||
|   +------------+           +------------+           +------------+|
|                                   |                               |
|                                   v                               |
|                             todos.json                            |
|                                                                   |
+-------------------------------------------------------------------+
```

---

## ความรู้ที่ใช้

| บท  | ความรู้           |
| --- | ----------------- |
| 2   | Variables & Types |
| 5   | Ownership         |
| 6   | Structs           |
| 7   | Enums             |
| 8   | Collections (Vec) |
| 9   | Error Handling    |
| 11  | Modules           |

---

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

👉 ต่อไป: [การออกแบบ](./02-design.md)
