# 📚 เรียน Rust ฉบับสมบูรณ์

[![Deploy GitHub Pages](https://github.com/premix-kernel/rust-tutorial/actions/workflows/deploy.yml/badge.svg)](https://github.com/premix-kernel/rust-tutorial/actions/workflows/deploy.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Language: Thai](https://img.shields.io/badge/Language-Thai%20🇹🇭-blue.svg)](https://premix-kernel.github.io/rust-tutorial)
[![Rust](https://img.shields.io/badge/Rust-1.83+-orange.svg)](https://www.rust-lang.org/)
[![mdBook](https://img.shields.io/badge/Made%20with-mdBook-blue.svg)](https://rust-lang.github.io/mdBook/)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-pink)](https://github.com/sponsors/premix-kernel)

> 🦀 เอกสารสอนภาษา Rust สำหรับผู้เริ่มต้น | คำอธิบายภาษาไทย | โค้ดภาษาอังกฤษ

## 🌐 อ่านออนไลน์

👉 **[อ่านหนังสือออนไลน์](https://premix-kernel.github.io/rust-tutorial)**

---

## ✨ Features

- 📖 **20 บทเรียน** ครอบคลุมตั้งแต่พื้นฐานถึง Advanced
- 📝 **100+ แบบฝึกหัด** พร้อมเฉลยละเอียด
- ❓ **100+ Quiz** ทดสอบความเข้าใจ
- 📋 **Rust Cheatsheet** สรุป syntax ครบ
- 🎯 **Rust Edition 2024** ทันสมัยที่สุด

---

## 🚀 เริ่มต้นใช้งาน

### วิธีที่ 1: อ่านออนไลน์ (แนะนำ)

เปิด [https://premix-kernel.github.io/rust-tutorial](https://premix-kernel.github.io/rust-tutorial)

### วิธีที่ 2: รันในเครื่อง

```bash
# Clone repository
git clone https://github.com/premix-kernel/rust-tutorial.git
cd rust-tutorial

# ติดตั้ง mdBook (ถ้ายังไม่มี)
cargo install mdbook

# รัน development server
cd docs
mdbook serve

# เปิด http://localhost:3000
```

### วิธีที่ 3: Build เป็น HTML

```bash
cd docs
mdbook build
# ไฟล์อยู่ใน docs/book/
```

### วิธีที่ 4: ทดสอบ Code Examples

```bash
cd docs
mdbook test
# ตรวจสอบว่า code blocks ถูกต้องก่อน commit
```

### วิธีที่ 5: รัน Example Code

```bash
# รันตัวอย่างโค้ด
cargo run --example hello_world      # บทที่ 1-4
cargo run --example ownership        # บทที่ 5
cargo run --example structs_enums    # บทที่ 6-7
cargo run --example collections      # บทที่ 8
cargo run --example error_handling   # บทที่ 9
cargo run --example generics_traits  # บทที่ 10
cargo run --example iterators        # บทที่ 13
cargo run --example smart_pointers   # บทที่ 14
cargo run --example concurrency      # บทที่ 15
cargo run --example async_await      # บทที่ 16
cargo run --example web_server       # บทที่ 19 (http://localhost:3000)
```

---

## 📖 เนื้อหา (20 บท)

### Part 1: พื้นฐาน

| บท  | หัวข้อ          | คำอธิบาย                         |
| --- | --------------- | -------------------------------- |
| 1   | Getting Started | ติดตั้ง Rust, Hello World, Cargo |
| 2   | Variables       | ตัวแปร, Data Types, Mutability   |
| 3   | Functions       | ฟังก์ชัน, Parameters, Return     |
| 4   | Control Flow    | if/else, loops, match            |

### Part 2: Core Concepts

| บท  | หัวข้อ           | คำอธิบาย                   |
| --- | ---------------- | -------------------------- |
| 5   | **Ownership** ⭐ | หัวใจของ Rust              |
| 6   | Structs          | โครงสร้างข้อมูล, Methods   |
| 7   | Enums            | Pattern Matching, Option   |
| 8   | Collections      | Vec, String, HashMap       |
| 9   | Error Handling   | panic!, Result, ? operator |

### Part 3: Advanced Concepts

| บท  | หัวข้อ            | คำอธิบาย                      |
| --- | ----------------- | ----------------------------- |
| 10  | Generics & Traits | Type parameters, Trait bounds |
| 11  | Modules           | Packages, Crates, Visibility  |
| 12  | Testing           | Unit tests, Integration tests |
| 13  | Iterators         | Closures, Iterator methods    |
| 14  | Smart Pointers    | Box, Rc, RefCell, Weak        |

### Part 4: Concurrency & Advanced

| บท  | หัวข้อ      | คำอธิบาย                 |
| --- | ----------- | ------------------------ |
| 15  | Concurrency | Threads, Channels, Mutex |
| 16  | Async/Await | Futures, Tokio           |
| 17  | Unsafe Rust | Raw pointers, FFI        |
| 18  | Macros      | Declarative, Procedural  |

### Part 5: Real World

| บท  | หัวข้อ          | คำอธิบาย       |
| --- | --------------- | -------------- |
| 19  | Web Development | Axum, REST API |
| 20  | Final Project   | CLI Todo App   |

### 📚 ภาคผนวก

- 📋 **Rust Cheatsheet** - สรุป syntax ที่ใช้บ่อย
- 🛠️ **Cargo Commands** - คำสั่ง Cargo ทั้งหมด
- 🔗 **Resources** - แหล่งเรียนรู้เพิ่มเติม
- ✍️ **แบบฝึกหัด** - 100+ ข้อพร้อมเฉลย
- ❓ **Quiz** - ทดสอบความเข้าใจ

---

## 🎯 กลุ่มเป้าหมาย

- 🆕 **ผู้เริ่มต้น** - ไม่เคยเขียนโปรแกรมมาก่อน
- 🔄 **Developers** - มีประสบการณ์ภาษาอื่นอยากเรียน Rust

---

## 🛠️ เครื่องมือที่แนะนำ

| เครื่องมือ | คำอธิบาย |
|------------|----------|
| [VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) | Editor ยอดนิยม พร้อม auto-complete |
| [RustRover](https://www.jetbrains.com/rust/) | JetBrains IDE สำหรับ Rust |
| [Rust Playground](https://play.rust-lang.org/) | ทดสอบโค้ดออนไลน์ |

---

## 📁 โครงสร้างโปรเจกต์

```
rust-tutorial/
├── .github/
│   └── workflows/
│       └── mdbook.yml      # CI/CD
├── docs/
│   ├── book.toml           # mdBook config
│   ├── book/               # Built HTML (gitignored)
│   └── src/
│       ├── SUMMARY.md      # สารบัญ
│       ├── introduction.md # หน้าแรก
│       ├── ch01-ch20/      # 20 บท
│       └── appendix/       # ภาคผนวก
├── .gitignore
├── Cargo.toml              # สำหรับ examples
├── CONTRIBUTING.md
├── LICENSE
└── README.md
```

---

## 🤝 Contributing

ยินดีรับทุกการมีส่วนร่วม! ดู [CONTRIBUTING.md](CONTRIBUTING.md) สำหรับรายละเอียด

---

## 📜 License

โปรเจกต์นี้อยู่ภายใต้ [MIT License](LICENSE)

---

## 🔗 Resources เพิ่มเติม

- 📖 [The Rust Book](https://doc.rust-lang.org/book/)
- 🎓 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🏋️ [Rustlings](https://github.com/rust-lang/rustlings)
- 📦 [Crates.io](https://crates.io)
- 💬 [Rust Discord](https://discord.gg/rust-lang)

---

<div align="center">

**🦀 Happy Coding with Rust!**

Made with ❤️ for Thai Rustaceans

</div>
