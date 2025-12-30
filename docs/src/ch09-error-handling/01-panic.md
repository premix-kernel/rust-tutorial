# panic! - Unrecoverable Errors

`panic!` หยุดโปรแกรมทันทีเมื่อเกิด error ที่จัดการไม่ได้

## เมื่อไหร่เกิด Panic?

### 1. เรียก panic! เอง

```rust
fn main() {
    panic!("crash and burn");
}
```

Output:

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

### 2. Bug ในโค้ด

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // 💥 panic! index out of bounds
}
```

---

## Backtrace

ดู backtrace เพื่อหาที่มาของ panic:

```bash
RUST_BACKTRACE=1 cargo run
```

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
stack backtrace:
   0: std::panicking::begin_panic_handler
   1: core::panicking::panic_bounds_check
   2: <usize as core::slice::SliceIndex<[T]>>::index
   3: playground::main
             at src/main.rs:3:5
```

---

## เมื่อไหร่ควรใช้ panic!

### ✅ ควรใช้

1. **Prototyping** - ตัวอย่างโค้ด, ทดลอง

```rust
fn main() {
    // ยังไม่ได้ implement
    todo!("implement this later");
}
```

2. **Tests** - เมื่อ test fail

```rust
#[test]
fn test_something() {
    assert_eq!(1, 2); // panic! ถ้าไม่เท่า
}
```

3. **Unrecoverable situation** - สถานการณ์ที่โปรแกรมต้องหยุด

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}
```

4. **Invalid state** - ข้อมูลอยู่ในสถานะที่ไม่ถูกต้อง

```rust,ignore
fn process_age(age: i32) {
    if age < 0 || age > 150 {
        panic!("Invalid age: {}", age);
    }
    // ...
}
```text

### ❌ ไม่ควรใช้

1. **Expected failures** - เช่น file not found, network error
2. **User input errors** - ผู้ใช้พิมพ์ผิด
3. **Recoverable errors** - ลอง retry ได้

> **Rule of thumb:** ถ้าผู้เรียกสามารถ handle error ได้ → ใช้ `Result`

> 💡 **Best Practices: panic! vs Result**
>
> ```text
> ┌─────────────────────────────────────────────────────────┐
> │                  เลือก Error Handling                   │
> ├─────────────────────────────────────────────────────────┤
> │  panic!           │  Result<T, E>                       │
> ├───────────────────┼─────────────────────────────────────┤
> │  ✓ Bug ในโค้ด     │  ✓ File not found                   │
> │  ✓ Invalid state  │  ✓ Network error                    │
> │  ✓ Tests          │  ✓ User input error                 │
> │  ✓ Prototyping    │  ✓ Parse error                      │
> │  ✓ Unrecoverable  │  ✓ Recoverable errors               │
> └───────────────────┴─────────────────────────────────────┘
> ```

---

## unwrap และ expect

### unwrap

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let value = x.unwrap(); // ✅ 5

    let y: Option<i32> = None;
    // let value = y.unwrap(); // 💥 panic!
}
```

### expect (ดีกว่า unwrap)

```rust,ignore
fn main() {
    let x: Option<i32> = None;

    // ให้ error message ที่ชัดเจน
    let value = x.expect("x should have a value");
}
```

Output:

```text
thread 'main' panicked at 'x should have a value', src/main.rs:4:18
```

---

## unreachable! และ todo!

```rust
fn main() {
    let level = 5;

    let description = match level {
        1..=5 => "beginner",
        6..=10 => "intermediate",
        11..=20 => "advanced",
        _ => unreachable!("level should be 1-20"),
    };

    // ยังไม่ได้ implement
    todo!("add more logic here");
}
```

---

## ตัวอย่างจริง: Assertion

```rust
fn set_age(age: u32) {
    assert!(age <= 150, "Age {} is unrealistic", age);
    println!("Age set to {}", age);
}

fn main() {
    set_age(25);  // ✅ OK
    set_age(200); // 💥 panic!
}
```

---

## ลองทำดู! 🎯

1. สร้าง function ที่ panic เมื่อได้รับค่าลบ
2. ใช้ expect แทน unwrap และให้ error message ที่ดี
3. ลองเปิด RUST_BACKTRACE=1 ดู backtrace

---

## สรุป

| Macro            | ใช้เมื่อ              |
| ---------------- | --------------------- |
| `panic!("msg")`  | Error ร้ายแรง         |
| `unreachable!()` | โค้ดที่ไม่ควรถูกเรียก |
| `todo!()`        | ยังไม่ได้ implement   |
| `assert!(cond)`  | ตรวจสอบเงื่อนไข       |

👉 ต่อไป: [Result<T, E>](./02-result.md)
