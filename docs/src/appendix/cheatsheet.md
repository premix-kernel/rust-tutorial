# Rust Cheatsheet

สรุปคำสั่งและ syntax ที่ใช้บ่อยใน Rust

---

## Variables

```rust
// Immutable (ค่าคงที่)
let x = 5;

// Mutable (เปลี่ยนค่าได้)
let mut y = 10;
y = 20;

// Type annotation
let z: i32 = 100;

// Constants (ต้องระบุ type)
const MAX_POINTS: u32 = 100_000;

// Shadowing
let x = 5;
let x = x + 1; // x = 6
```text

---

## Data Types

### Scalar Types

| Type                      | ตัวอย่าง        |
| ------------------------- | --------------- |
| `i8, i16, i32, i64, i128` | `-128`, `42`    |
| `u8, u16, u32, u64, u128` | `0`, `255`      |
| `f32, f64`                | `3.14`, `2.0`   |
| `bool`                    | `true`, `false` |
| `char`                    | `'a'`, `'🦀'`   |

### Compound Types

```rust
// Tuple
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup;  // destructuring
let first = tup.0;     // indexing

// Array (fixed size)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];
let arr = [3; 5]; // [3, 3, 3, 3, 3]
```

---

## Functions

```rust
// Basic function
fn greet() {
    println!("Hello!");
}

// With parameters
fn greet_name(name: &str) {
    println!("Hello, {}!", name);
}

// With return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon = return
}

// Early return
fn check(x: i32) -> bool {
    if x > 10 {
        return true;
    }
    false
}
```

---

## Control Flow

### If/Else

```rust
if x > 5 {
    println!("Big");
} else if x > 0 {
    println!("Small");
} else {
    println!("Zero or negative");
}

// As expression
let result = if x > 5 { "big" } else { "small" };
```

### Loops

```rust
// Infinite loop
loop {
    break;  // exit
}

// While loop
while x > 0 {
    x -= 1;
}

// For loop
for i in 0..5 {
    println!("{}", i);  // 0, 1, 2, 3, 4
}

for item in &vec {
    println!("{}", item);
}
```

---

## Ownership

```rust
// Move
let s1 = String::from("hello");
let s2 = s1;  // s1 is invalid now

// Clone (deep copy)
let s1 = String::from("hello");
let s2 = s1.clone();  // s1 still valid

// Reference (borrow)
let s1 = String::from("hello");
let len = calculate_length(&s1);  // s1 still valid

// Mutable reference
fn change(s: &mut String) {
    s.push_str(" world");
}
```

---

## Structs

```rust
// Define
struct User {
    username: String,
    email: String,
    active: bool,
}

// Create
let user = User {
    username: String::from("john"),
    email: String::from("john@example.com"),
    active: true,
};

// Methods
impl User {
    fn new(username: String) -> Self {
        Self {
            username,
            email: String::new(),
            active: true,
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}
```

---

## Enums & Match

```rust
// Define
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

// Match
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Write: {}", text),
}

// Option
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

match x {
    Some(value) => println!("{}", value),
    None => println!("No value"),
}

// if let
if let Some(value) = x {
    println!("{}", value);
}
```

---

## Error Handling

```rust
// Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Handle Result
match divide(10.0, 2.0) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Error: {}", e),
}

// ? operator
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}

// unwrap (panic if error)
let content = std::fs::read_to_string("file.txt").unwrap();

// expect (panic with message)
let content = std::fs::read_to_string("file.txt")
    .expect("Failed to read file");
```

---

## Collections

### Vec

```rust
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

v.push(4);
v.pop();

let first = &v[0];
let first = v.get(0);  // Option<&i32>

for i in &v {
    println!("{}", i);
}
```

### String

```rust
let mut s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

s.push_str(" world");
s.push('!');

let s3 = format!("{} {}", s1, s2);
```

### HashMap

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", "value");

let value = map.get("key");  // Option<&V>

for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

---

## Iterators

```rust
let v = vec![1, 2, 3, 4, 5];

// Common methods
v.iter().map(|x| x * 2);
v.iter().filter(|x| **x > 2);
v.iter().sum::<i32>();
v.iter().collect::<Vec<_>>();
v.iter().for_each(|x| println!("{}", x));
v.iter().find(|x| **x == 3);
v.iter().any(|x| *x > 2);
v.iter().all(|x| *x > 0);
```

---

## Smart Pointers

```rust
// Box - Heap allocation
let b = Box::new(5);

// Rc - Reference counting
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);

// RefCell - Interior mutability
use std::cell::RefCell;
let data = RefCell::new(5);
*data.borrow_mut() += 1;
```

---

## Traits

```rust
// Define
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn default_summary(&self) -> String {
        String::from("...")
    }
}

// Implement
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// where clause
fn notify<T>(item: &T)
where
    T: Summary + Clone,
{
    println!("{}", item.summarize());
}
```

---

## Async

```rust,ignore
// Async function
async fn fetch_data() -> String {
    // ...
    String::from("data")
}

// Await
async fn process() {
    let data = fetch_data().await;
    println!("{}", data);
}

// Tokio main
#[tokio::main]
async fn main() {
    process().await;
}
```

---

👉 [Cargo Commands](./cargo-commands.md)

```