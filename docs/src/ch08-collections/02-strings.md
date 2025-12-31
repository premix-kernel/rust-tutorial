# String

**String** ใน Rust มี 2 แบบหลัก:

| Type     | คำอธิบาย                       |
| -------- | ------------------------------ |
| `String` | Owned, mutable, heap-allocated |
| `&str`   | Borrowed, immutable, slice     |

---

## สร้าง String

```rust
fn main() {
    // จาก string literal
    let s1 = String::from("Hello");
    let s2 = "Hello".to_string();

    // String ว่าง
    let s3 = String::new();

    // จาก format!
    let s4 = format!("Hello, {}!", "World");

    println!("{}, {}, {}, {}", s1, s2, s3, s4);
}
```

---

## String vs &str

```rust
fn main() {
    let s1: String = String::from("Hello"); // owned
    let s2: &str = "Hello";                 // borrowed (string literal)
    let s3: &str = &s1;                     // borrowed from String

    // String -> &str (automatic coercion)
    greet(&s1);
    greet(s2);

    // &str -> String
    let s4: String = s2.to_string();
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

> **แนะนำ:** ใช้ `&str` เป็น parameter, ใช้ `String` เมื่อต้อง own ข้อมูล

---

## ต่อ String

### push_str และ push

```rust
fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World"); // ต่อ string
    s.push('!');           // ต่อ char

    println!("{}", s); // Hello, World!
}
```

### + Operator

```rust,compile_fail
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    let s3 = s1 + &s2; // s1 ถูก move!

    // println!("{}", s1); // ❌ Error: s1 was moved
    println!("{}", s2); // ✅ OK
    println!("{}", s3); // Hello, World!
}
```

> **หมายเหตุ:** `+` รับ `String` ซ้าย และ `&str` ขวา

### format! (แนะนำ)

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    // format! ไม่ move อะไรเลย
    let s3 = format!("{}, {}!", s1, s2);

    println!("{}", s1); // ✅ OK
    println!("{}", s2); // ✅ OK
    println!("{}", s3); // Hello, World!
}
```

---

## UTF-8 และการ Index

**String ใน Rust เป็น UTF-8** → ไม่สามารถ index ด้วย `s[0]` ได้!

```rust,compile_fail
fn main() {
    let hello = String::from("สวัสดี");

    // let first = hello[0]; // ❌ Error!

    // ใช้ slicing ได้ (แต่ระวัง!)
    // let first_byte = &hello[0..1]; // 💥 panic! กลาง character
    let first_char = &hello[0..3]; // ✅ "ส" (3 bytes)

    println!("First char: {}", first_char);
}
```

### การวนลูป

```rust
fn main() {
    let hello = String::from("สวัสดี");

    // วนตาม characters
    for c in hello.chars() {
        println!("{}", c);
    }

    // วนตาม bytes
    for b in hello.bytes() {
        println!("{}", b);
    }
}
```

---

## Methods ที่ใช้บ่อย

```rust
fn main() {
    let s = String::from("  Hello, World!  ");

    // Length
    println!("len: {}", s.len());              // 18 (bytes)
    println!("chars: {}", s.chars().count()); // 18 (characters)

    // trim
    println!("trimmed: '{}'", s.trim());

    // contains, starts_with, ends_with
    println!("contains 'World': {}", s.contains("World"));

    // replace
    println!("replaced: {}", s.replace("World", "Rust"));

    // split
    for word in s.trim().split(',') {
        println!("word: '{}'", word.trim());
    }

    // to_uppercase, to_lowercase
    println!("upper: {}", s.to_uppercase());
    println!("lower: {}", s.to_lowercase());
}
```

---

## ตัวอย่างจริง: Parse และ Format

```rust
fn main() {
    // Parse string to number
    let num_str = "42";
    let num: i32 = num_str.parse().expect("Not a number");
    println!("Parsed: {}", num);

    // Format number to string
    let formatted = format!("The answer is {}", num);
    println!("{}", formatted);

    // Format with padding
    let padded = format!("{:0>5}", num); // "00042"
    println!("Padded: {}", padded);
}
```

---

## ลองทำดู! 🎯

1. สร้าง function ที่นับคำใน string
2. สร้าง function ที่ reverse string
3. สร้าง function ที่ตรวจสอบว่าเป็น palindrome หรือไม่

---

## สรุป

| แนวคิด  | ตัวอย่าง                 |
| ------- | ------------------------ |
| Create  | `String::from("hello")`  |
| Append  | `s.push_str("world")`    |
| Concat  | `format!("{}{}", a, b)`  |
| Iterate | `for c in s.chars()`     |
| Slice   | `&s[0..5]` (ระวัง UTF-8) |

👉 ต่อไป: [HashMap](./03-hashmaps.md)
