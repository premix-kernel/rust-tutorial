# if let & while let

`if let` และ `while let` เป็นทางเลือกที่กระชับสำหรับ `match` เมื่อสนใจแค่ pattern เดียว

## ปัญหา: Match ยาวไป

```rust,ignore
fn main() {
    let some_value: Option<i32> = Some(3);

    // ยาวเกินไปสำหรับแค่ pattern เดียว
    match some_value {
        Some(value) => println!("Value: {}", value),
        _ => (),  // ไม่ทำอะไร
    }
}
```

## if let

```rust,ignore
fn main() {
    let some_value: Option<i32> = Some(3);

    // กระชับกว่า!
    if let Some(value) = some_value {
        println!("Value: {}", value);
    }
}
```

---

## Syntax

```rust
if let PATTERN = EXPRESSION {
    // ทำเมื่อ match
}
```

หรือพร้อม else:

```rust
if let PATTERN = EXPRESSION {
    // ทำเมื่อ match
} else {
    // ทำเมื่อไม่ match
}
```

---

## if let กับ else

```rust,ignore
fn main() {
    let some_value: Option<i32> = None;

    if let Some(value) = some_value {
        println!("Value: {}", value);
    } else {
        println!("No value!");
    }
}
```

เทียบเท่ากับ:

```rust,ignore
match some_value {
    Some(value) => println!("Value: {}", value),
    _ => println!("No value!"),
}
```

---

## if let กับ Enum

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    if let Message::Move { x, y } = msg {
        println!("Moving to ({}, {})", x, y);
    }

    let msg2 = Message::Write(String::from("hello"));

    if let Message::Write(text) = msg2 {
        println!("Message: {}", text);
    }
}
```

---

## while let

ทำซ้ำตราบใดที่ pattern ยัง match:

```rust
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // pop() returns Option<T>
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

Output:

```
3
2
1
```

---

## ตัวอย่าง: Iterator

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();

    while let Some(num) = iter.next() {
        println!("{}", num);
    }
}
```

---

## let else (Rust 1.65+)

Unwrap หรือ return เร็ว:

```rust
fn get_length(s: Option<String>) -> usize {
    let Some(text) = s else {
        return 0;  // ต้อง return, break, continue, panic
    };

    text.len()
}

fn main() {
    println!("{}", get_length(Some(String::from("hello")))); // 5
    println!("{}", get_length(None)); // 0
}
```

---

## เปรียบเทียบ

| `match`             | `if let`       |
| ------------------- | -------------- |
| ครอบคลุมทุก pattern | pattern เดียว  |
| Exhaustive          | ไม่ exhaustive |
| ยาวกว่า             | กระชับกว่า     |

### ใช้ match เมื่อ:

- มีหลาย patterns
- ต้องการให้ compiler บังคับครบทุกกรณี

### ใช้ if let เมื่อ:

- สนใจแค่ pattern เดียว
- ต้องการความกระชับ

---

## ตัวอย่างจริง

```rust
fn main() {
    let config_max: Option<u8> = Some(100);

    // if let - กระชับ
    if let Some(max) = config_max {
        println!("Maximum is {}", max);
    }

    // Regular if - ใช้ไม่ได้!
    // if config_max.is_some() {
    //     let max = config_max.unwrap();  // clunky
    //     println!("Maximum is {}", max);
    // }
}
```

---

## Chained if let

```rust
fn main() {
    let maybe_number: Option<i32> = Some(42);
    let maybe_string: Option<&str> = Some("hello");

    if let Some(n) = maybe_number {
        if let Some(s) = maybe_string {
            println!("Number: {}, String: {}", n, s);
        }
    }
}
```

หรือใช้ `&&` (Rust 1.53+):

```rust
fn main() {
    let maybe_number: Option<i32> = Some(42);
    let maybe_string: Option<&str> = Some("hello");

    if let (Some(n), Some(s)) = (maybe_number, maybe_string) {
        println!("Number: {}, String: {}", n, s);
    }
}
```

---

## ลองทำดู! 🎯

1. เขียน if let สำหรับ `Option<String>`
2. ใช้ while let pop จาก Vec
3. แปลง match ที่มี 2 arms เป็น if let else

---

## สรุปบทที่ 7

| แนวคิด    | ตัวอย่าง                                  |
| --------- | ----------------------------------------- |
| Enum      | `enum Name { A, B(T) }`                   |
| Option    | `Some(x)`, `None`                         |
| match     | `match x { A => ..., B => ... }`          |
| if let    | `if let Some(x) = opt { ... }`            |
| while let | `while let Some(x) = iter.next() { ... }` |

👉 ต่อไป: [บทที่ 8: Collections](../ch08-collections/README.md)
