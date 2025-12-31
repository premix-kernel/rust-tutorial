# Loops - การวนซ้ำ

Rust มี 3 ประเภทของ loop:

| Loop    | ใช้เมื่อ                     |
| ------- | ---------------------------- |
| `loop`  | วนไม่รู้จบ จนกว่าจะ break    |
| `while` | วนตราบใดที่เงื่อนไขเป็น true |
| `for`   | วนตามจำนวนที่กำหนด           |

---

## loop - วนไม่รู้จบ

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count >= 3 {
            break; // ออกจาก loop
        }
    }
}
```

ผลลัพธ์:

```text
Count: 1
Count: 2
Count: 3
```
```