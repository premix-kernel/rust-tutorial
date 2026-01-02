# Macros ที่ใช้บ่อย

รวม macros ที่ใช้บ่อยใน Rust standard library

## Output Macros

### println! และ print!

```rust
fn main() {
    // Basic
    println!("Hello, World!");
    print!("No newline");
    println!(" - now with newline");

    // With placeholders
    println!("Value: {}", 42);
    println!("Multiple: {} and {}", 1, 2);

    // Named arguments
    println!("{name} is {age} years old", name = "Alice", age = 30);

    // Debug format
    let v = vec![1, 2, 3];
    println!("{:?}", v);    // [1, 2, 3]
    println!("{:#?}", v);   // Pretty print

    // Formatting
    println!("{:>10}", "right");   // "     right"
    println!("{:<10}", "left");    // "left      "
    println!("{:^10}", "center");  // "  center  "
    println!("{:0>5}", 42);        // "00042"
    println!("{:.2}", 3.14159);    // "3.14"
}
```

### eprintln! และ eprint!

พิมพ์ไปยัง stderr:

```rust
fn main() {
    eprintln!("Error: something went wrong!");
    eprint!("Warning: ");
    eprintln!("check your input");
}
```

---

## format!

สร้าง String โดยไม่พิมพ์:

```rust
fn main() {
    let s = format!("Hello, {}!", "World");
    println!("{}", s);

    let num = 42;
    let formatted = format!("Number: {:05}", num);  // "Number: 00042"

    // Useful for building strings
    let mut log = String::new();
    log.push_str(&format!("[INFO] Started at {}\n", "10:00"));
    log.push_str(&format!("[INFO] Finished at {}\n", "10:05"));
    println!("{}", log);
}
```

---

## vec!

สร้าง Vec:

```rust
fn main() {
    // With elements
    let v1 = vec![1, 2, 3, 4, 5];

    // Repeat value
    let v2 = vec![0; 5];  // [0, 0, 0, 0, 0]

    // Empty with type
    let v3: Vec<String> = vec![];

    // With expressions
    let v4 = vec![1 + 1, 2 + 2, 3 + 3];  // [2, 4, 6]

    println!("{:?}", v1);
}
```

---

## Assertion Macros

### assert!

```rust
fn main() {
    assert!(true);
    assert!(1 + 1 == 2);

    // With message
    let x = 5;
    assert!(x > 0, "x must be positive!");
    assert!(x > 0, "x was {}, expected positive", x);
}
```

### assert_eq! และ assert_ne!

```rust
fn main() {
    let a = 4;
    let b = 2 + 2;

    assert_eq!(a, b);     // passes
    assert_ne!(a, 5);     // passes

    // With message
    assert_eq!(a, b, "Expected {} to equal {}", a, b);
}
```

### debug_assert!

เฉพาะ debug builds:

```rust
fn process(x: i32) {
    // ถูกลบใน release builds
    debug_assert!(x > 0, "x must be positive in debug mode");
    println!("Processing {}", x);
}

fn main() {
    process(5);
}
```

---

## dbg!

Debug print พร้อม file/line:

```rust
fn main() {
    let x = 5;
    dbg!(x);  // [src/main.rs:3] x = 5

    // Returns the value
    let y = dbg!(x * 2) + 1;  // [src/main.rs:6] x * 2 = 10
    dbg!(y);  // [src/main.rs:7] y = 11

    // Multiple values
    let a = 1;
    let b = 2;
    dbg!(a, b, a + b);
}
```

---

## todo! และ unimplemented!

Placeholder สำหรับโค้ดที่ยังไม่เสร็จ:

```rust
fn not_done_yet() -> i32 {
    todo!("implement this function")  // panics with message
}

fn old_way() -> i32 {
    unimplemented!("use new_way() instead")
}

fn main() {
    // not_done_yet();  // would panic
}
```

---

## unreachable!

สำหรับโค้ดที่ไม่ควรถึง:

```rust
fn divide(a: i32, b: i32) -> i32 {
    match b {
        0 => panic!("division by zero"),
        _ => a / b,
    }
}

fn process(x: i32) {
    match x {
        1..=100 => println!("Valid"),
        _ if x > 100 => println!("Too big"),
        _ if x < 1 => println!("Too small"),
        _ => unreachable!("all cases covered"),
    }
}
```

---

## panic!

หยุดโปรแกรมทันที:

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    if v.is_empty() {
        panic!("Vector is empty!");
    }

    // With format
    let index = 10;
    if index >= v.len() {
        panic!("Index {} out of bounds for length {}", index, v.len());
    }
}
```

---

## include_str! และ include_bytes!

รวมไฟล์ใน binary:

```rust,ignore
// Include as string
const README: &str = include_str!("../../README.md");

// Include as bytes
const LOGO: &[u8] = include_bytes!("../../logo.png");

fn main() {
    println!("First 100 chars of README:");
    println!("{}", &README[..100.min(README.len())]);

    println!("Logo size: {} bytes", LOGO.len());
}
```

---

## concat! และ stringify!

Compile-time string operations:

```rust
const VERSION: &str = concat!("v", "1", ".", "0");

fn main() {
    println!("Version: {}", VERSION);  // "v1.0"

    // stringify! converts expression to string literal
    let x = 1 + 2;
    println!("{} = {}", stringify!(1 + 2), x);  // "1 + 2 = 3"
}
```

---

## cfg!

Check compile-time configuration:

```rust
fn main() {
    if cfg!(target_os = "windows") {
        println!("Running on Windows");
    } else if cfg!(target_os = "linux") {
        println!("Running on Linux");
    } else if cfg!(target_os = "macos") {
        println!("Running on macOS");
    }

    if cfg!(debug_assertions) {
        println!("Debug mode");
    } else {
        println!("Release mode");
    }
}
```

---

## ลองทำดู! 🎯

1. ใช้ format! สร้าง formatted string
2. ใช้ dbg! debug expression
3. ใช้ cfg! ตรวจสอบ OS

---

## สรุปบทที่ 18

| Macro          | Purpose         | Returns       |
| -------------- | --------------- | ------------- |
| `println!`     | Print to stdout | `()`          |
| `eprintln!`    | Print to stderr | `()`          |
| `format!`      | Create String   | `String`      |
| `vec!`         | Create Vec      | `Vec<T>`      |
| `assert!`      | Check condition | `()` or panic |
| `assert_eq!`   | Check equality  | `()` or panic |
| `dbg!`         | Debug print     | Same as input |
| `todo!`        | Placeholder     | panic         |
| `panic!`       | Stop program    | never         |
| `include_str!` | Include file    | `&str`        |
| `cfg!`         | Check config    | `bool`        |

👉 ต่อไป: [บทที่ 19: Web Development](../ch19-web-development/README.md)
