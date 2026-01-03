# Packages & Crates

## Crate

**Crate** คือหน่วยการ compile ที่เล็กที่สุด มี 2 แบบ:

- **Binary crate** - โปรแกรมที่รันได้ (มี `main`)
- **Library crate** - โค้ดที่ใช้ร่วมกัน (ไม่มี `main`)

### 📦 Package Structure Visualization

```text
+-------------------------------------------------------------------+
|                    Rust Package Hierarchy                         |
+-------------------------------------------------------------------+
|                                                                   |
|   Package (Cargo.toml)                                            |
|   +-- Binary Crate (src/main.rs)      <--- executable             |
|   |   +-- mod auth                                                |
|   |   +-- mod database                                            |
|   |   +-- mod routes                                              |
|   |                                                               |
|   +-- Library Crate (src/lib.rs)      <--- shared code            |
|       +-- pub mod models                                          |
|       +-- pub mod utils                                           |
|       +-- pub mod errors                                          |
|                                                                   |
+-------------------------------------------------------------------+
|   Dependencies (Cargo.toml)                                       |
|   +-- serde = "1.0"                   <--- External crates        |
|   +-- tokio = { version = "1", features = ["full"] }              |
|   +-- rand = "0.8"                                                |
|                                                                   |
+-------------------------------------------------------------------+
```

---

## Package

**Package** = collection ของ crates พร้อม `Cargo.toml`

```text
my-package/
├── Cargo.toml
├── src/
│   ├── main.rs      # binary crate root
│   └── lib.rs       # library crate root (optional)
└── src/bin/         # additional binaries
    └── another.rs
```

---

## สร้าง Package

```bash
# Binary package
cargo new my-app

# Library package
cargo new my-lib --lib
```

---

## Cargo.toml

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

👉 ต่อไป: [Modules](./02-modules.md)
