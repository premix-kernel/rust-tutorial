# References & Borrowing

**References** ช่วยให้เรา "ยืม" ค่า **โดยไม่ย้าย ownership**

## ปัญหา: ต้อง Return ค่ากลับ

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("Length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len) // ต้อง return s กลับ 😩
}
```

## ทางออก: ใช้ Reference

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // ส่ง reference

    println!("Length of '{}' is {}", s1, len); // s1 ยังใช้ได้!
}

fn calculate_length(s: &String) -> usize { // รับ reference
    s.len()
}
```

---

## Reference คืออะไร?

Reference คือ pointer ที่ชี้ไปยังค่า โดยไม่เป็นเจ้าของ:

```
s1 ──────► "hello" (Heap)
           ▲
           │
&s1 ───────┘ (Reference ชี้ไปที่ s1)
```

- `&s1` สร้าง reference ไปยัง `s1`
- `&String` คือ type ของ reference

---

## กฎของ References (สำคัญมาก!)

> **กฎ 1:** ในเวลาใดก็ตาม มี**หนึ่งใน**สิ่งนี้ได้:
>
> - **หนึ่ง** mutable reference (`&mut T`)
> - **กี่อันก็ได้** immutable references (`&T`)
>
> **กฎ 2:** References ต้อง valid เสมอ (ไม่ dangling)

---

## Immutable References (`&T`)

สามารถมีหลาย immutable references พร้อมกันได้:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s; // ✅
    let r2 = &s; // ✅
    let r3 = &s; // ✅

    println!("{}, {}, {}", r1, r2, r3);
}
```

### แต่ไม่สามารถแก้ไขได้

```rust
fn main() {
    let s = String::from("hello");

    let r = &s;

    // r.push_str(" world"); // ❌ Error! cannot borrow as mutable
}
```

---

## Mutable References (`&mut T`)

ใช้ `&mut` เมื่อต้องการแก้ไข:

```rust
fn main() {
    let mut s = String::from("hello"); // ต้อง mut

    change(&mut s); // ส่ง mutable reference

    println!("{}", s); // "hello world"
}

fn change(s: &mut String) {
    s.push_str(" world");
}
```

### มีได้แค่หนึ่ง mutable reference

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // ❌ Error! cannot borrow as mutable more than once

    println!("{}", r1);
}
```

---

## ห้ามผสม Mutable และ Immutable

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // immutable borrow
    let r2 = &s;     // immutable borrow
    // let r3 = &mut s; // ❌ Error! cannot borrow as mutable

    println!("{} and {}", r1, r2);
}
```

### แต่ทำได้ถ้า scope ไม่ทับกัน

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 และ r2 ไม่ถูกใช้อีกหลังจากนี้

    let r3 = &mut s; // ✅ OK เพราะ r1, r2 จบแล้ว
    println!("{}", r3);
}
```

---

## Dangling References

Rust ป้องกัน dangling references:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // ❌ Error!
    let s = String::from("hello");

    &s // reference ไปยัง s
} // s ถูก drop ที่นี่ -> reference จะ invalid!
```

### แก้ไขโดย return ค่าแทน

```rust
fn main() {
    let s = no_dangle();
    println!("{}", s);
}

fn no_dangle() -> String { // ✅ Return String แทน
    let s = String::from("hello");
    s // move ownership ออกไป
}
```

---

## สรุป: Reference Types

| Type     | ความหมาย            | ตัวอย่าง      |
| -------- | ------------------- | ------------- |
| `&T`     | Immutable reference | `&String`     |
| `&mut T` | Mutable reference   | `&mut String` |

---

## ตัวอย่างจริง

```rust
fn main() {
    let mut message = String::from("Hello");

    // อ่านค่า
    print_length(&message);

    // แก้ไขค่า
    add_world(&mut message);

    println!("Final: {}", message);
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn add_world(s: &mut String) {
    s.push_str(", World!");
}
```

---

## ลองทำดู! 🎯

1. เขียน function ที่รับ `&String` และ return ความยาว
2. เขียน function ที่รับ `&mut Vec<i32>` และ push ค่าเข้าไป
3. ลองสร้าง dangling reference และดู error

---

## สรุป

| แนวคิด      | กฎ                              |
| ----------- | ------------------------------- |
| `&T`        | อ่านได้อย่างเดียว, มีหลายอันได้ |
| `&mut T`    | แก้ไขได้, มีได้อันเดียว         |
| ห้ามผสม     | &T และ &mut T พร้อมกันไม่ได้    |
| No dangling | Reference ต้อง valid เสมอ       |

👉 ต่อไป: [Slices](./04-slices.md)
