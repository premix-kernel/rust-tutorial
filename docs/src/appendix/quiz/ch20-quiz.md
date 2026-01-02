# Quiz: บทที่ 20 - Final Project

## คำถาม 20.1

`clap` crate ใช้ทำอะไร?

A. Web server  
B. Command line argument parsing  
C. Database  
D. Testing

<details>
<summary>ดูเฉลย</summary>

**B. Command line argument parsing**

```rust,ignore
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    name: String,
}
```

</details>

---

## คำถาม 20.2

`serde` ใช้ทำอะไร?

A. Networking  
B. Serialization/Deserialization  
C. Logging  
D. Compression

<details>
<summary>ดูเฉลย</summary>

**B. Serialization/Deserialization**

```rust,ignore
#[derive(Serialize, Deserialize)]
struct Data { /* ... */ }
```

</details>

---

## คำถาม 20.3

`anyhow` crate ใช้ทำอะไร?

A. Async runtime  
B. Easy error handling  
C. Web framework  
D. Database ORM

<details>
<summary>ดูเฉลย</summary>

**B. Easy error handling**

```rust,ignore
fn main() -> anyhow::Result<()> {
    // ใช้ ? ได้กับทุก error types
    Ok(())
}
```

</details>

---

## คำถาม 20.4

Project structure ที่ดีควรมีอะไร?

A. ทุกอย่างใน main.rs  
B. แยก modules ตาม responsibility  
C. ไม่ใช้ modules  
D. ใส่ tests ใน main.rs

<details>
<summary>ดูเฉลย</summary>

**B. แยก modules ตาม responsibility**

```
src/
├── main.rs
├── lib.rs
├── models/
├── handlers/
└── utils/
```

</details>

---

## คำถาม 20.5

`cargo build --release` ต่างจาก `cargo build` อย่างไร?

A. ไม่ต่าง  
B. Optimized, production-ready  
C. Debug mode  
D. รันเร็วกว่า

<details>
<summary>ดูเฉลย</summary>

**B. Optimized, production-ready**

- `cargo build` = debug, fast compile
- `cargo build --release` = optimized, slower compile

</details>

---

🎉 **ยินดีด้วย! คุณทำ Quiz ครบทุกบทแล้ว!**

👉 [กลับหน้าแรก](../../introduction.md)
