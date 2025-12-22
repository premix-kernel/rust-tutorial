# Associated Functions

**Associated Functions** คือ functions ใน `impl` block ที่ไม่มี `self` เรียกด้วย `::`

## Syntax

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50); // ใช้ :: ไม่ใช่ .

    println!("{:?}", rect);
}
```

---

## Constructor Pattern

Associated functions มักใช้เป็น **constructor**:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Constructor หลัก
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Constructor สำหรับสี่เหลี่ยมจัตุรัส
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Constructor default
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
        }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let square = Rectangle::square(10);
    let default = Rectangle::default();

    println!("{:?}", rect);
    println!("{:?}", square);
    println!("{:?}", default);
}
```

> **หมายเหตุ:** ใช้ `Self` แทนชื่อ struct ได้ใน impl block

---

## Methods vs Associated Functions

| Methods                         | Associated Functions         |
| ------------------------------- | ---------------------------- |
| มี `&self`, `&mut self`, `self` | ไม่มี self                   |
| เรียกด้วย `.`                   | เรียกด้วย `::`               |
| เข้าถึง instance data ได้       | เข้าถึง instance data ไม่ได้ |

```rust
impl Rectangle {
    // Associated Function (ไม่มี self)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method (มี &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(10, 20); // ::
    let area = rect.area();             // .
}
```

---

## ตัวอย่าง: String::from

`String::from` เป็น associated function ที่เราใช้บ่อย:

```rust
fn main() {
    let s = String::from("hello"); // associated function

    let len = s.len(); // method
}
```

---

## ตัวอย่างจริง: User

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    // Constructor
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }

    // สร้างจาก email (extract username)
    fn from_email(email: String) -> Self {
        let username = email
            .split('@')
            .next()
            .unwrap_or("unknown")
            .to_string();

        Self::new(username, email)
    }

    // Methods
    fn is_active(&self) -> bool {
        self.active
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let user1 = User::new(
        String::from("alice"),
        String::from("alice@example.com")
    );

    let user2 = User::from_email(String::from("bob@example.com"));

    println!("{:?}", user1);
    println!("{:?}", user2);
}
```

---

## Builder Pattern

ใช้ associated functions และ methods ร่วมกัน:

```rust
#[derive(Debug)]
struct Car {
    brand: String,
    model: String,
    year: u32,
    color: String,
}

impl Car {
    fn builder() -> CarBuilder {
        CarBuilder::default()
    }
}

#[derive(Default)]
struct CarBuilder {
    brand: String,
    model: String,
    year: u32,
    color: String,
}

impl CarBuilder {
    fn brand(mut self, brand: &str) -> Self {
        self.brand = brand.to_string();
        self
    }

    fn model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    fn year(mut self, year: u32) -> Self {
        self.year = year;
        self
    }

    fn color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    fn build(self) -> Car {
        Car {
            brand: self.brand,
            model: self.model,
            year: self.year,
            color: self.color,
        }
    }
}

fn main() {
    let car = Car::builder()
        .brand("Toyota")
        .model("Camry")
        .year(2024)
        .color("Blue")
        .build();

    println!("{:?}", car);
}
```

---

## ลองทำดู! 🎯

1. สร้าง `Point::origin()` ที่ return Point(0, 0)
2. สร้าง `Circle::with_radius(r)` constructor
3. Implement builder pattern สำหรับ struct ที่คุณสร้าง

---

## สรุปบทที่ 6

| แนวคิด              | ตัวอย่าง                      |
| ------------------- | ----------------------------- |
| Struct              | `struct Name { field: Type }` |
| Method              | `fn method(&self)`            |
| Associated Function | `fn func() -> Self`           |
| Constructor         | `fn new(...) -> Self`         |
| Builder             | Chain methods ที่ return Self |

👉 ต่อไป: [บทที่ 7: Enums & Pattern Matching](../ch07-enums/README.md)
