# Return Values (ค่าที่ส่งกลับ)

ฟังก์ชันสามารถ **ส่งค่ากลับ** ให้ผู้เรียกใช้ได้

## Syntax

```rust
fn function_name() -> ReturnType {
    // return value
}
```

- ใช้ `->` ระบุชนิดค่าที่ส่งกลับ
- ค่าสุดท้ายใน function body จะถูกส่งกลับ (ไม่ต้องใส่ `;`)

---

## ตัวอย่างพื้นฐาน

```rust
fn main() {
    let x = five();
    println!("x = {}", x); // 5
}

fn five() -> i32 {
    5 // ไม่มี ; = ส่งค่านี้กลับ
}
```

---

## Expressions vs Statements

ใน Rust มีแนวคิดสำคัญ:

| ประเภท         | คำอธิบาย                  | มีค่า? |
| -------------- | ------------------------- | ------ |
| **Statement**  | ทำบางอย่าง ไม่ return ค่า | ❌     |
| **Expression** | ประมวลผลและ return ค่า    | ✅     |

```rust
fn main() {
    // Statement - ไม่มีค่า
    let x = 5; // let ... เป็น statement

    // Expression - มีค่า
    let y = {
        let a = 3;
        a + 1   // ไม่มี ; = expression ที่ return ค่า
    };

    println!("y = {}", y); // 4
}
```

> **สำคัญมาก:** ถ้าใส่ `;` ท้าย จะกลายเป็น statement และไม่ return ค่า!

```rust
fn main() {
    let y = {
        let a = 3;
        a + 1; // มี ; = statement = return () (unit)
    };

    // y มีค่าเป็น () ไม่ใช่ 4!
}
```

---

## ตัวอย่างฟังก์ชันที่ return ค่า

```rust
fn main() {
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum); // 8

    let product = multiply(4, 7);
    println!("4 * 7 = {}", product); // 28

    let result = add(multiply(2, 3), 4);
    println!("(2 * 3) + 4 = {}", result); // 10
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

---

## ใช้ return keyword

สามารถใช้ `return` เพื่อ return ก่อนจบฟังก์ชันได้:

```rust
fn main() {
    println!("{}", absolute(-5)); // 5
    println!("{}", absolute(3));  // 3
}

fn absolute(x: i32) -> i32 {
    if x < 0 {
        return -x; // return ก่อนกำหนด
    }
    x // implicit return
}
```

> **แนะนำ:** ใช้ implicit return (ไม่ใส่ `return`) เมื่อเป็นไปได้
> ใช้ `return` เมื่อต้อง return ก่อนจบฟังก์ชัน

---

## ฟังก์ชันที่ไม่ return ค่า

ถ้าไม่ระบุ `->` ฟังก์ชันจะ return `()` (unit):

```rust
fn main() {
    let result = print_hello();
    println!("result = {:?}", result); // ()
}

fn print_hello() {
    println!("Hello!");
    // implicitly returns ()
}
```

เทียบเท่ากับ:

```rust
fn print_hello() -> () {
    println!("Hello!");
}
```

---

## ตัวอย่างจริง

```rust
fn main() {
    let radius = 5.0;
    let area = circle_area(radius);
    let circumference = circle_circumference(radius);

    println!("Circle with radius {}:", radius);
    println!("  Area: {:.2}", area);
    println!("  Circumference: {:.2}", circumference);
}

fn circle_area(radius: f64) -> f64 {
    3.14159 * radius * radius
}

fn circle_circumference(radius: f64) -> f64 {
    2.0 * 3.14159 * radius
}
```

ผลลัพธ์:

```text
Circle with radius 5:
  Area: 78.54
  Circumference: 31.42
```