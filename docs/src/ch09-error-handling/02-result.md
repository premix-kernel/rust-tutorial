# Result<T, E>

`Result` ใช้จัดการ **recoverable errors** - error ที่สามารถ handle ได้

## นิยาม

```rust
enum Result<T, E> {
    Ok(T),   // สำเร็จ มีค่า T
    Err(E),  // ล้มเหลว มี error E
}
```

---

## ตัวอย่างพื้นฐาน

```rust
use std::fs::File;

fn main() {
    let result = File::open("hello.txt");

    let file = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file: {:?}", error);
        }
    };

    println!("File opened: {:?}", file);
}
```

---

## จัดการหลายประเภท Error

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let result = File::open("hello.txt");

    let file = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // สร้างไฟล์ใหม่ถ้าไม่มี
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Cannot create file: {:?}", e),
                }
            }
            other_error => {
                panic!("Cannot open file: {:?}", other_error);
            }
        },
    };

    println!("File: {:?}", file);
}
```

---

## Shortcuts: unwrap และ expect

### unwrap

```rust
use std::fs::File;

fn main() {
    // panic ถ้า error
    let file = File::open("hello.txt").unwrap();
}
```

### expect (แนะนำ)

```rust
use std::fs::File;

fn main() {
    // panic พร้อม message
    let file = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}
```

---

## Methods ที่ใช้บ่อย

```rust
fn main() {
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");

    // is_ok, is_err
    println!("ok is_ok: {}", ok_result.is_ok());   // true
    println!("err is_err: {}", err_result.is_err()); // true

    // unwrap_or
    println!("ok: {}", ok_result.unwrap_or(0));   // 42
    println!("err: {}", err_result.unwrap_or(0)); // 0

    // unwrap_or_else
    let value = err_result.unwrap_or_else(|e| {
        println!("Error was: {}", e);
        -1
    });
    println!("value: {}", value); // -1

    // map - transform Ok value
    let doubled = ok_result.map(|x| x * 2);
    println!("doubled: {:?}", doubled); // Ok(84)

    // map_err - transform Err value
    let new_err = err_result.map_err(|e| format!("Error: {}", e));
    println!("new_err: {:?}", new_err); // Err("Error: error")
}
```

---

## ok() และ err()

แปลง Result เป็น Option:

```rust
fn main() {
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");

    // ok() -> Option<T>
    println!("{:?}", ok_result.ok());   // Some(42)
    println!("{:?}", err_result.ok());  // None

    // err() -> Option<E>
    println!("{:?}", ok_result.err());  // None
    println!("{:?}", err_result.err()); // Some("error")
}
```

---

## and_then (Chaining)

```rust
fn square(x: i32) -> Result<i32, &'static str> {
    if x > 100 {
        Err("Too large to square")
    } else {
        Ok(x * x)
    }
}

fn main() {
    let result = Ok(5)
        .and_then(square)  // Ok(25)
        .and_then(square); // Ok(625)

    println!("{:?}", result); // Ok(625)

    let result2 = Ok(50)
        .and_then(square)   // Ok(2500)
        .and_then(square);  // Err("Too large to square")

    println!("{:?}", result2);
}
```

---

## ตัวอย่างจริง: Parse Number

```rust
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

fn main() {
    match parse_and_double("42") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_and_double("abc") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## ลองทำดู! 🎯

1. เขียน function `divide(a, b) -> Result<f64, String>` ที่ return Err เมื่อ b = 0
2. ใช้ `and_then` เพื่อ chain หลาย operations
3. แปลง string เป็น number แล้วบวกเลขสองตัว

---

## สรุป

| Method               | คำอธิบาย                        |
| -------------------- | ------------------------------- |
| `unwrap()`           | ดึงค่า หรือ panic               |
| `expect(msg)`        | ดึงค่า หรือ panic พร้อม message |
| `unwrap_or(default)` | ดึงค่า หรือใช้ default          |
| `map(f)`             | แปลง Ok value                   |
| `and_then(f)`        | Chain operations                |
| `ok()`               | แปลงเป็น Option                 |

👉 ต่อไป: [การส่งต่อ Error](./03-propagating.md)
