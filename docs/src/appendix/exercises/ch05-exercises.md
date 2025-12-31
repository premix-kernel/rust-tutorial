# แบบฝึกหัด: บทที่ 5 - Ownership

## แบบฝึกหัดที่ 1: Move

โค้ดนี้มี error อะไร? แก้ไขให้ทำงานได้

```rust,ignore
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
}
```text

<details>
<summary>ดูเฉลย</summary>

**Error:** `s1` ถูก move ไป `s2` แล้ว

**วิธีแก้ 1: ใช้ clone**

```rust,ignore
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // ✅ OK
}
```text

**วิธีแก้ 2: ใช้ reference**

```rust,ignore
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}", s1); // ✅ OK
}
```text

</details>

---

## แบบฝึกหัดที่ 2: Function Ownership

โค้ดนี้มี error อะไร? แก้ไขให้ทำงานได้

```rust,compile_fail,ignore
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    print_string(s);
    println!("{}", s); // Error!
}
```text

<details>
<summary>ดูเฉลย</summary>

**Error:** `s` ถูก move เข้า function แล้ว

**วิธีแก้: ใช้ reference**

```rust,ignore
fn print_string(s: &String) {  // รับ reference
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    print_string(&s);  // ส่ง reference
    println!("{}", s); // ✅ OK
}
```text

</details>

---

## แบบฝึกหัดที่ 3: Mutable Reference

เขียน function `append_world` ที่:

- รับ mutable reference ของ String
- เพิ่ม " World" ต่อท้าย

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn append_world(s: &mut String) {
    s.push_str(" World");
}

fn main() {
    let mut greeting = String::from("Hello");
    append_world(&mut greeting);
    println!("{}", greeting); // Hello World
}
```text

</details>

---

## แบบฝึกหัดที่ 4: Borrowing Rules

โค้ดนี้ถูกหรือผิด? อธิบาย

```rust,ignore
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println!("{}, {}, {}", r1, r2, r3);
}
```

<details>
<summary>ดูเฉลย</summary>

**ผิด!** ไม่สามารถมี mutable reference พร้อมกับ immutable reference ได้

**วิธีแก้:**

```rust,ignore
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);  // ใช้ r1, r2 ก่อน

    let r3 = &mut s;  // แล้วค่อยสร้าง mutable ref
    println!("{}", r3);
}
```text

</details>

---

## แบบฝึกหัดที่ 5: Slice

เขียน function `first_word` ที่:

- รับ `&String`
- Return slice ของคำแรก (ก่อน space แรก)

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("Hello World");
    let first = first_word(&s);
    println!("First word: {}", first); // Hello
}
```

</details>

---

👉 [บทที่ 6](./ch06-exercises.md)

```