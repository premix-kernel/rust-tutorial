# Methods

**Methods** คือ functions ที่ผูกกับ struct ใช้ `impl` block

## นิยาม Method

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {}", rect.area());
}
```

---

## &self Parameter

`&self` เป็น shorthand สำหรับ `self: &Self`:

| Parameter   | ความหมาย             |
| ----------- | -------------------- |
| `&self`     | ยืม (อ่านอย่างเดียว) |
| `&mut self` | ยืมแบบแก้ไขได้       |
| `self`      | รับ ownership        |

```rust
impl Rectangle {
    // อ่านค่าอย่างเดียว
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // แก้ไขค่า
    fn double_width(&mut self) {
        self.width *= 2;
    }

    // รับ ownership (หลังเรียก instance จะใช้ไม่ได้)
    fn destroy(self) {
        println!("Destroying {:?}", self);
    }
}
```

---

## Methods ที่มี Parameters

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2)); // true
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3)); // false
}
```

---

## หลาย impl Blocks

สามารถแยก methods เป็นหลาย `impl` blocks ได้:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

---

## Method Chaining

ออกแบบ methods ที่ return `&mut self` เพื่อ chain ได้:

```rust,ignore
#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { value: 0 }
    }

    fn increment(&mut self) -> &mut Self {
        self.value += 1;
        self
    }

    fn add(&mut self, n: i32) -> &mut Self {
        self.value += n;
        self
    }
}

fn main() {
    let mut counter = Counter::new();

    counter
        .increment()
        .increment()
        .add(10);

    println!("{:?}", counter); // Counter { value: 12 }
}
```

---

## Automatic Referencing

Rust จะเพิ่ม `&`, `&mut`, หรือ `*` อัตโนมัติเมื่อเรียก method:

```rust
fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    // เทียบเท่ากัน:
    rect.area();
    (&rect).area();
}
```

---

## ตัวอย่างจริง: Circle

```rust
use std::f64::consts::PI;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn grow(&mut self, amount: f64) {
        self.radius += amount;
    }
}

fn main() {
    let mut circle = Circle { radius: 5.0 };

    println!("Radius: {}", circle.radius);
    println!("Area: {:.2}", circle.area());
    println!("Circumference: {:.2}", circle.circumference());

    circle.grow(2.5);
    println!("After grow - Area: {:.2}", circle.area());
}
```

---

## ลองทำดู! 🎯

1. เพิ่ม method `is_square(&self)` ให้ Rectangle
2. สร้าง struct `BankAccount` พร้อม methods deposit, withdraw
3. สร้าง method chain สำหรับ builder pattern

---

## สรุป

| แนวคิด     | ตัวอย่าง                         |
| ---------- | -------------------------------- |
| Method     | `fn method(&self) {}`            |
| Mutable    | `fn method(&mut self) {}`        |
| With args  | `fn method(&self, arg: Type) {}` |
| impl block | `impl StructName { ... }`        |

👉 ต่อไป: [Associated Functions](./03-associated-functions.md)
