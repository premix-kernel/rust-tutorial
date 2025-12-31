# การสร้าง Enum

## Syntax พื้นฐาน

```rust,ignore
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    // ...
}
```

---

## Enum กับ Data

Variants สามารถมีข้อมูลแนบได้:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

---

## Variants หลายแบบ

แต่ละ variant มีข้อมูลต่างกันได้:

```rust
enum Message {
    Quit,                       // ไม่มีข้อมูล
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // 3 integers
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
}
```

---

## Methods บน Enum

เหมือนกับ struct:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
        println!("Message received!");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

---

## เปรียบเทียบ Enum vs Struct

### ใช้ Struct + Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
}
```

### ใช้ Enum alone (ดีกว่า!)

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
}
```

---

## ตัวอย่างจริง: WebEvent

```rust
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: {}", s),
        WebEvent::Click { x, y } => println!("Clicked at ({}, {})", x, y),
    }
}

fn main() {
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(press);
    inspect(click);
}
```

---

## ลองทำดู! 🎯

1. สร้าง enum `TrafficLight` (Red, Yellow, Green)
2. สร้าง enum `Shape` ที่มี Circle(radius), Rectangle(w, h)
3. เพิ่ม method `area()` ให้ Shape

---

## สรุป

| แนวคิด       | ตัวอย่าง                     |
| ------------ | ---------------------------- |
| Basic enum   | `enum Name { A, B }`         |
| With data    | `enum Name { A(i32) }`       |
| Named fields | `enum Name { A { x: i32 } }` |
| Use          | `Name::A`                    |

👉 ต่อไป: [Option\<T\>](./02-option.md)
