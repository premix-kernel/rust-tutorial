# Cargo Commands

คำสั่ง Cargo ที่ใช้บ่อย

---

## 🚀 พื้นฐาน

| คำสั่ง                     | คำอธิบาย                           |
| -------------------------- | ---------------------------------- |
| `cargo new project_name`   | สร้างโปรเจกต์ใหม่ (binary)         |
| `cargo new --lib lib_name` | สร้างโปรเจกต์ใหม่ (library)        |
| `cargo init`               | สร้างโปรเจกต์ในโฟลเดอร์ปัจจุบัน    |
| `cargo build`              | Build โปรเจกต์ (debug)             |
| `cargo build --release`    | Build โปรเจกต์ (release/optimized) |
| `cargo run`                | Build และ Run                      |
| `cargo run --release`      | Run แบบ release                    |

---

## 🧪 Testing

| คำสั่ง                           | คำอธิบาย            |
| -------------------------------- | ------------------- |
| `cargo test`                     | รัน tests ทั้งหมด   |
| `cargo test test_name`           | รัน test ที่ชื่อตรง |
| `cargo test -- --show-output`    | แสดง println!       |
| `cargo test -- --test-threads=1` | รัน sequential      |
| `cargo test -- --ignored`        | รัน ignored tests   |
| `cargo test --doc`               | รัน doc tests       |

---

## 📦 Dependencies

| คำสั่ง                            | คำอธิบาย             |
| --------------------------------- | -------------------- |
| `cargo add crate_name`            | เพิ่ม dependency     |
| `cargo add tokio --features full` | เพิ่มพร้อม features  |
| `cargo remove crate_name`         | ลบ dependency        |
| `cargo update`                    | อัปเดต dependencies  |
| `cargo tree`                      | แสดง dependency tree |

---

## 📋 ตรวจสอบ

| คำสั่ง              | คำอธิบาย                               |
| ------------------- | -------------------------------------- |
| `cargo check`       | Check ว่า compile ได้ (เร็วกว่า build) |
| `cargo clippy`      | Lint หา improvements                   |
| `cargo fmt`         | Format code                            |
| `cargo fmt --check` | Check format                           |
| `cargo audit`       | ตรวจ security vulnerabilities          |

---

## 📖 Documentation

| คำสั่ง                | คำอธิบาย               |
| --------------------- | ---------------------- |
| `cargo doc`           | สร้าง documentation    |
| `cargo doc --open`    | สร้างและเปิดใน browser |
| `cargo doc --no-deps` | ไม่รวม dependencies    |

---

## 🔧 Advanced

| คำสั่ง                       | คำอธิบาย             |
| ---------------------------- | -------------------- |
| `cargo clean`                | ลบ target directory  |
| `cargo publish`              | Publish ไป crates.io |
| `cargo install crate_name`   | ติดตั้ง binary       |
| `cargo uninstall crate_name` | ลบ binary            |
| `cargo bench`                | Run benchmarks       |

---

## 🎯 Examples

```bash
# สร้างโปรเจกต์ใหม่
cargo new my_project
cd my_project

# เพิ่ม dependencies
cargo add serde --features derive
cargo add tokio --features full

# Build และ Run
cargo run

# ตรวจสอบ code
cargo check
cargo clippy
cargo fmt

# รัน tests
cargo test

# Build สำหรับ production
cargo build --release

# สร้าง documentation
cargo doc --open
```

---

## 📁 Examples

```bash
# รัน example
cargo run --example example_name

# รัน specific binary
cargo run --bin binary_name

# รัน workspace member
cargo run -p package_name
```

---

## 🛠️ Workspace

```bash
# Cargo.toml (root)
[workspace]
members = [
    "crate1",
    "crate2",
]

# Build all
cargo build --workspace

# Test all
cargo test --workspace

# Run specific
cargo run -p crate1
```

---

## 🔍 Useful Flags

| Flag                    | คำอธิบาย             |
| ----------------------- | -------------------- |
| `--verbose` หรือ `-v`   | แสดงรายละเอียด       |
| `--quiet` หรือ `-q`     | ลดการแสดงผล          |
| `--jobs N` หรือ `-j N`  | จำนวน parallel jobs  |
| `--target triple`       | Cross compile        |
| `--features "f1 f2"`    | เปิด features        |
| `--all-features`        | เปิดทุก features     |
| `--no-default-features` | ปิด default features |

---

👉 [Resources](./resources.md)
