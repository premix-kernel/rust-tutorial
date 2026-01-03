# Option\<T\>

`Option<T>` เป็น enum มาตรฐานที่ใช้แทน **null** ในภาษาอื่น

## ปัญหาของ Null

ในภาษาอื่น `null` ทำให้เกิด bugs มากมาย:

```javascript
// JavaScript
let name = null;
console.log(name.length); // 💥 Crash!
```

Rust ไม่มี `null` แต่ใช้ `Option<T>` แทน

---

## นิยามของ Option

```rust
enum Option<T> {
    None,    // ไม่มีค่า
    Some(T), // มีค่า
}
```

> **หมายเหตุ:** `Option`, `Some`, `None` อยู่ใน prelude ใช้ได้เลยโดยไม่ต้อง import

---

## การใช้งาน Option

```rust,ignore
fn main() {
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);  // Some(5)
    println!("{:?}", some_string);  // Some("hello")
    println!("{:?}", absent_number); // None
}
```

---

## ทำไม Option ดีกว่า Null?

### Compiler บังคับให้จัดการ

```rust,ignore
fn main() {
    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y; // ❌ Error! cannot add i32 and Option<i32>
}
```

ต้อง "แกะ" Option ก่อนใช้ → บังคับให้คิดเรื่อง None

---

## การจัดการ Option

### 1. match

```rust,ignore
fn main() {
    let x: Option<i32> = Some(5);

    match x {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }
}
```

### 2. if let

```rust,ignore
fn main() {
    let x: Option<i32> = Some(5);

    if let Some(value) = x {
        println!("Value: {}", value);
    }
}
```

### 3. unwrap (ระวัง!)

```rust,ignore
fn main() {
    let x: Option<i32> = Some(5);
    let value = x.unwrap(); // ✅ ได้ 5

    let y: Option<i32> = None;
    // let value2 = y.unwrap(); // 💥 Panic!
}
```

### 4. unwrap_or (ปลอดภัยกว่า)

```rust,ignore
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("{}", x.unwrap_or(0)); // 5
    println!("{}", y.unwrap_or(0)); // 0 (default)
}
```

### 5. map

```rust,ignore
fn main() {
    let x: Option<i32> = Some(5);

    let doubled = x.map(|v| v * 2);

    println!("{:?}", doubled); // Some(10)
}
```

---

## Methods ที่ใช้บ่อย

| Method               | คำอธิบาย                   |
| -------------------- | -------------------------- |
| `is_some()`          | return true ถ้า Some       |
| `is_none()`          | return true ถ้า None       |
| `unwrap()`           | ดึงค่าออก (panic ถ้า None) |
| `unwrap_or(default)` | ดึงค่า หรือใช้ default     |
| `unwrap_or_else(f)`  | ดึงค่า หรือ call function  |
| `map(f)`             | แปลงค่าข้างใน              |
| `and_then(f)`        | chain Options              |

```rust,ignore
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is_some: {}", x.is_some()); // true
    println!("y is_none: {}", y.is_none()); // true

    // Chain operations
    let result = x
        .map(|v| v * 2)      // Some(10)
        .map(|v| v + 1)      // Some(11)
        .unwrap_or(0);       // 11

    println!("Result: {}", result);
}
```

---

## ตัวอย่างจริง: หาค่าใน Array

```rust,ignore
fn find_item(items: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in items.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    match find_item(&numbers, 3) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}
```

---

## ตัวอย่างจริง: Division

```rust,ignore
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    println!("{:?}", result1); // Some(5.0)
    println!("{:?}", result2); // None
}
```

---

## ลองทำดู! 🎯

1. เขียน function `first(vec: &Vec<i32>) -> Option<i32>`
2. เขียน function `parse_number(s: &str) -> Option<i32>`
3. Chain หลาย Option methods ด้วยกัน

---

## สรุป

| แนวคิด    | ตัวอย่าง                                    |
| --------- | ------------------------------------------- |
| Some      | `Some(5)`                                   |
| None      | `None`                                      |
| Match     | `match opt { Some(x) => ..., None => ... }` |
| unwrap_or | `opt.unwrap_or(default)`                    |
| map       | `opt.map(\|x\| x * 2)`                      |

👉 ต่อไป: [Match Expression](./03-match.md)
