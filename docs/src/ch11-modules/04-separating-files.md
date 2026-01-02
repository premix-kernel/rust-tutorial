# แยกไฟล์

จัดโครงสร้างโปรเจกต์ใหญ่ด้วยการแยก modules เป็นไฟล์

## 2 รูปแบบโครงสร้าง

### รูปแบบ 1: ไฟล์เดียว

```text
src/
├── main.rs
└── my_module.rs   ← mod my_module; จะหาที่นี่
```

### รูปแบบ 2: โฟลเดอร์ + mod.rs

```text
src/
├── main.rs
└── my_module/
    ├── mod.rs      ← mod my_module; จะหาที่นี่
    └── submodule.rs
```

---

## ตัวอย่าง: โปรเจกต์จริง

### โครงสร้าง

```text
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── config.rs
    ├── database/
    │   ├── mod.rs
    │   ├── connection.rs
    │   └── queries.rs
    └── handlers/
        ├── mod.rs
        ├── users.rs
        └── posts.rs
```

### src/lib.rs

```rust,ignore
// ประกาศ modules
pub mod config;
pub mod database;
pub mod handlers;
```

### src/config.rs

```rust,ignore
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Config {
            database_url: String::from("postgres://localhost/mydb"),
            port: 3000,
        }
    }
}
```

### src/database/mod.rs

```rust,ignore
// ประกาศ submodules
pub mod connection;
pub mod queries;

// Re-export items ที่ใช้บ่อย
pub use connection::DatabaseConnection;
pub use queries::Query;
```

### src/database/connection.rs

```rust,ignore
pub struct DatabaseConnection {
    url: String,
    connected: bool,
}

impl DatabaseConnection {
    pub fn new(url: &str) -> Self {
        DatabaseConnection {
            url: url.to_string(),
            connected: false,
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
        println!("Connected to {}", self.url);
    }
}
```

### src/database/queries.rs

```rust,ignore
use super::DatabaseConnection;

pub struct Query {
    sql: String,
}

impl Query {
    pub fn new(sql: &str) -> Self {
        Query { sql: sql.to_string() }
    }

    pub fn execute(&self, _conn: &DatabaseConnection) {
        println!("Executing: {}", self.sql);
    }
}
```

### src/handlers/mod.rs

```rust,ignore
pub mod users;
pub mod posts;
```

### src/main.rs

```rust,ignore
use my_project::{config::Config, database::DatabaseConnection};

fn main() {
    let config = Config::new();
    let mut db = DatabaseConnection::new(&config.database_url);
    db.connect();

    println!("Server starting on port {}", config.port);
}
```

---

## Module Discovery Rules

Rust หา module ตามลำดับนี้:

1. **Inline:** `mod name { ... }` ใน file เดียวกัน
2. **File:** `src/name.rs`
3. **Directory:** `src/name/mod.rs`

```rust,ignore
// src/lib.rs
mod foo;        // หา src/foo.rs หรือ src/foo/mod.rs
mod bar;        // หา src/bar.rs หรือ src/bar/mod.rs

mod inline {    // inline module
    pub fn stuff() {}
}
```

---

## 2018+ Edition Style (แนะนำ)

แทนที่จะใช้ `mod.rs` ใช้ชื่อ folder:

```text
src/
├── lib.rs
├── database.rs      ← แทน database/mod.rs
└── database/
    ├── connection.rs
    └── queries.rs
```

```rust,ignore
// src/database.rs
pub mod connection;
pub mod queries;
```

---

## Private vs Public Files

```rust,ignore
// src/lib.rs
mod private_module;      // private - ใช้ได้ใน crate นี้เท่านั้น
pub mod public_module;   // public - user ของ crate ใช้ได้
```

---

## Workspace

สำหรับโปรเจกต์ใหญ่มาก:

```text
my_workspace/
├── Cargo.toml           ← [workspace]
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── api/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── cli/
│       ├── Cargo.toml
│       └── src/main.rs
```

```toml,ignore
# Cargo.toml (workspace root)
[workspace]
members = ["crates/*"]
```

---

## ลองทำดู! 🎯

1. สร้างโปรเจกต์ที่มี 2-3 modules
2. แยก modules เป็นไฟล์
3. ใช้ `pub use` re-export

---

## สรุปบทที่ 11

| Pattern        | Location                           |
| -------------- | ---------------------------------- |
| `mod foo`      | `src/foo.rs` หรือ `src/foo/mod.rs` |
| `mod foo::bar` | `src/foo/bar.rs`                   |
| Inline         | `mod foo { ... }`                  |

### Best Practices

1. ใช้ 2018 style (ไม่ใช้ `mod.rs`)
2. `pub use` re-export items ที่สำคัญ
3. ใช้ workspace สำหรับโปรเจกต์ใหญ่

👉 ต่อไป: [บทที่ 12: Testing](../ch12-testing/README.md)
