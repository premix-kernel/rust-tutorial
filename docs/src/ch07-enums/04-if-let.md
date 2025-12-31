# if let & while let

`if let` และ `while let` เป็นทางเลือกที่กระชับสำหรับ `match` เมื่อสนใจแค่ pattern เดียว

## ปัญหา: Match ยาวไป

```rust
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

```rust
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

```rust
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

```rust
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

```text
3
2
1
```
```