# Match เบื้องต้น

`match` เป็น control flow ที่ทรงพลังใน Rust ใช้เปรียบเทียบค่ากับ patterns

## Syntax พื้นฐาน

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

---

## ตัวอย่างพื้นฐาน

```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}
```

ผลลัพธ์:

```text
Three
```