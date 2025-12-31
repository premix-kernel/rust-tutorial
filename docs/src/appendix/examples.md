# Example Code

โค้ดตัวอย่างที่สามารถ clone และรันได้เลย

---

## 📦 วิธีใช้

```bash
# Clone repository
git clone https://github.com/premix-kernel/rust-tutorial.git
cd rust-tutorial

# รันตัวอย่าง
cargo run --example <name>
```

---

## 📋 รายการ Examples

| Example           | บท  | คำอธิบาย                       |
| ----------------- | --- | ------------------------------ |
| `hello_world`     | 1-4 | ตัวแปร, ฟังก์ชัน, Control Flow |
| `ownership`       | 5   | Move, Clone, Borrowing, Slices |
| `structs_enums`   | 6-7 | Structs, Methods, Enums, Match |
| `collections`     | 8   | Vec, String, HashMap           |
| `error_handling`  | 9   | Result, ?, unwrap_or           |
| `generics_traits` | 10  | Generics, Traits, Bounds       |
| `iterators`       | 13  | Closures, map, filter, fold    |
| `smart_pointers`  | 14  | Box, Rc, RefCell, Weak         |
| `concurrency`     | 15  | Threads, Channels, Mutex       |
| `async_await`     | 16  | async/await, join!, spawn      |
| `web_server`      | 19  | Axum REST API                  |

---

## 🚀 ตัวอย่างคำสั่ง

```bash
# Hello World
cargo run --example hello_world

# Ownership (สำคัญมาก!)
cargo run --example ownership

# Web Server (เปิด http://localhost:3000)
cargo run --example web_server
```

---

## 📝 โครงสร้างไฟล์

```text
examples/
├── hello_world.rs       # บทที่ 1-4
├── ownership.rs         # บทที่ 5
├── structs_enums.rs     # บทที่ 6-7
├── collections.rs       # บทที่ 8
├── error_handling.rs    # บทที่ 9
├── generics_traits.rs   # บทที่ 10
├── iterators.rs         # บทที่ 13
├── smart_pointers.rs    # บทที่ 14
├── concurrency.rs       # บทที่ 15
├── async_await.rs       # บทที่ 16
└── web_server.rs        # บทที่ 19
```

---

## ✨ Output ตัวอย่าง

### hello_world

```
🦀 สวัสดี Rust!
Hello, World!

📝 ตัวอย่างตัวแปร:
Name: Rustacean
Age: 25
Is Learning: true
```

### ownership

```
🦀 Ownership Demo

1️⃣ Move:
s2 = hello

2️⃣ Clone:
s3 = world, s4 = world

3️⃣ Copy:
x = 5, y = 5
```

---

👉 [กลับหน้า Appendix](./README.md)
