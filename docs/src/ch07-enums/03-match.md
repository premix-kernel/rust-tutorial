# Match Expression

`match` เป็น control flow ที่ทรงพลังใน Rust ใช้เปรียบเทียบค่ากับหลาย patterns

## Syntax

```rust,ignore
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default,
}
```

---

## Match กับ Enum

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Value: {} cents", value_in_cents(coin));
}
```

---

## Exhaustive Matching

`match` **ต้องครอบคลุมทุกกรณี**:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn describe(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        // ❌ Error: pattern `Blue` not covered
    }
}
```

### แก้ไข

```rust,ignore
fn describe(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
```

---

## \_ (Catch-all)

ใช้ `_` จับ patterns ที่เหลือ:

```rust
fn main() {
    let number = 13;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}
```

---

## Match กับ Binding

ดึงค่าจาก enum:

```rust,ignore
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Text: {}", text);
        }
    }
}

fn main() {
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("Hello")));
}
```

---

## Match กับ Option

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);  // Some(6)
    println!("{:?}", none); // None
}
```

---

## Match Guards

เพิ่มเงื่อนไขให้ pattern:

```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than 5: {}", x),
        Some(x) => println!("5 or more: {}", x),
        None => println!("None"),
    }
}
```

---

## @ Binding

จับค่าพร้อมทั้งตรวจ pattern:

```rust
fn main() {
    let msg = Message::Move { x: 5, y: 10 };

    match msg {
        Message::Move { x: x_val @ 0..=10, y } => {
            println!("x ({}) is in range, y = {}", x_val, y);
        }
        Message::Move { x, y } => {
            println!("x ({}) is out of range, y = {}", x, y);
        }
        _ => {}
    }
}

enum Message {
    Move { x: i32, y: i32 },
}
```

---

## Multiple Patterns

ใช้ `|` เพื่อ match หลาย patterns:

```rust
fn main() {
    let n = 3;

    match n {
        1 | 2 | 3 => println!("One, two, or three"),
        4..=10 => println!("Four to ten"),
        _ => println!("Something else"),
    }
}
```

---

## Match เป็น Expression

```rust
fn main() {
    let x = 1;

    let message = match x {
        1 => "one",
        2 => "two",
        _ => "other",
    };

    println!("{}", message);
}
```

---

## ตัวอย่างจริง: Command Processing

```rust,ignore
enum Command {
    Start,
    Stop,
    Speed(u32),
    Position { x: i32, y: i32 },
}

fn execute(cmd: Command) -> String {
    match cmd {
        Command::Start => String::from("Starting..."),
        Command::Stop => String::from("Stopping..."),
        Command::Speed(s) if s > 100 => format!("Too fast! {}", s),
        Command::Speed(s) => format!("Setting speed to {}", s),
        Command::Position { x, y } => format!("Moving to ({}, {})", x, y),
    }
}

fn main() {
    println!("{}", execute(Command::Start));
    println!("{}", execute(Command::Speed(50)));
    println!("{}", execute(Command::Speed(150)));
    println!("{}", execute(Command::Position { x: 10, y: 20 }));
}
```

---

## ลองทำดู! 🎯

1. เขียน match สำหรับ `Option<String>` ที่ print ความยาว
2. เขียน match ที่ใช้ guard เช็คค่าบวก/ลบ
3. เขียน function ที่ return String ด้วย match

---

## สรุป

| แนวคิด    | ตัวอย่าง                         |
| --------- | -------------------------------- |
| Basic     | `match x { 1 => ..., _ => ... }` |
| Binding   | `Some(value) => ...`             |
| Multiple  | `1 \| 2 \| 3 => ...`             |
| Guard     | `Some(x) if x > 0 => ...`        |
| @ Binding | `x @ 1..=10 => ...`              |

👉 ต่อไป: [if let & while let](./04-if-let.md)
