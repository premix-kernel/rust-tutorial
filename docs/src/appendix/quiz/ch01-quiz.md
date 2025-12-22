# Quiz: บทที่ 1 - Getting Started

## คำถามที่ 1

คำสั่งใดใช้ตรวจสอบว่า Rust ติดตั้งสำเร็จ?

A. `rust --version`  
B. `rustc --version`  
C. `cargo version`  
D. `rust-check`

<details>
<summary>ดูเฉลย</summary>

**B. `rustc --version`**

`rustc` คือ Rust compiler ส่วน `cargo --version` ก็ใช้ได้เช่นกัน

</details>

---

## คำถามที่ 2

Cargo คืออะไร?

A. Text editor สำหรับ Rust  
B. Rust compiler  
C. Package manager และ build tool  
D. Testing framework

<details>
<summary>ดูเฉลย</summary>

**C. Package manager และ build tool**

Cargo ทำหน้าที่:

- จัดการ dependencies
- Build โปรเจกต์
- รัน tests
- สร้าง documentation

</details>

---

## คำถามที่ 3

คำสั่งใดสร้างโปรเจกต์ Rust ใหม่?

A. `cargo init project`  
B. `cargo new project`  
C. `rustc new project`  
D. `rust create project`

<details>
<summary>ดูเฉลย</summary>

**B. `cargo new project`**

- `cargo new` สร้างโฟลเดอร์ใหม่
- `cargo init` สร้างในโฟลเดอร์ปัจจุบัน

</details>

---

## คำถามที่ 4

ไฟล์ใดเก็บ dependencies ของโปรเจกต์?

A. `main.rs`  
B. `Cargo.lock`  
C. `Cargo.toml`  
D. `package.json`

<details>
<summary>ดูเฉลย</summary>

**C. `Cargo.toml`**

- `Cargo.toml` = ประกาศ dependencies
- `Cargo.lock` = lock versions ที่ใช้จริง

</details>

---

## คำถามที่ 5

คำสั่งใดตรวจสอบโค้ดโดยไม่ build?

A. `cargo run`  
B. `cargo build`  
C. `cargo test`  
D. `cargo check`

<details>
<summary>ดูเฉลย</summary>

**D. `cargo check`**

`cargo check` เร็วกว่า `cargo build` เพราะไม่สร้าง binary

</details>

---

👉 [Quiz บทที่ 2](./ch02-quiz.md)
