# แบบฝึกหัด: บทที่ 7 - Enums & Pattern Matching

## แบบฝึกหัดที่ 1: สร้าง Enum

สร้าง enum `Direction` ที่มี North, South, East, West

<details>
<summary>ดูเฉลย</summary>

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;

    match dir {
        Direction::North => println!("ไปทางเหนือ"),
        Direction::South => println!("ไปทางใต้"),
        Direction::East => println!("ไปทางตะวันออก"),
        Direction::West => println!("ไปทางตะวันตก"),
    }
}
```

</details>

---

## แบบฝึกหัดที่ 2: Enum with Data

สร้าง enum `Message` ที่มี:

- `Quit` (ไม่มีข้อมูล)
- `Move { x: i32, y: i32 }`
- `Write(String)`

<details>
<summary>ดูเฉลย</summary>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}
```

</details>

---

## แบบฝึกหัดที่ 3: Option

เขียน function `divide` ที่ return `Option<f64>`:

- Return `None` ถ้าหาร 0
- Return `Some(result)` ถ้าหารได้

<details>
<summary>ดูเฉลย</summary>

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero"),
    }

    // หรือใช้ if let
    if let Some(result) = divide(10.0, 0.0) {
        println!("Result: {}", result);
    } else {
        println!("Cannot divide by zero");
    }
}
```

</details>

---

## แบบฝึกหัดที่ 4: Match with Guards

เขียน function ที่รับ `Option<i32>` และ:

- ถ้า `Some(n)` และ n > 0: print "positive"
- ถ้า `Some(n)` และ n < 0: print "negative"
- ถ้า `Some(0)`: print "zero"
- ถ้า `None`: print "no value"

<details>
<summary>ดูเฉลย</summary>

```rust
fn describe(value: Option<i32>) {
    match value {
        Some(n) if n > 0 => println!("positive"),
        Some(n) if n < 0 => println!("negative"),
        Some(0) => println!("zero"),
        None => println!("no value"),
        _ => unreachable!(),
    }
}

fn main() {
    describe(Some(5));   // positive
    describe(Some(-3));  // negative
    describe(Some(0));   // zero
    describe(None);      // no value
}
```

</details>

---

## แบบฝึกหัดที่ 5: if let

แปลง match นี้เป็น if let:

```rust
match some_value {
    Some(x) => println!("{}", x),
    None => (),
}
```

<details>
<summary>ดูเฉลย</summary>

```rust
if let Some(x) = some_value {
    println!("{}", x);
}
```

</details>

---

👉 [บทที่ 8](./ch08-exercises.md)
