# Custom Error Types

สร้าง error types เองเพื่อให้มีข้อมูลมากขึ้นและจัดการได้ดีขึ้น

## Simple String Error

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Enum Error Type

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(MathError::DivisionByZero) => println!("Cannot divide by zero!"),
        Err(e) => println!("Other error: {:?}", e),
    }
}
```

---

## Implement Display และ Error

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    InvalidInput(String),
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(item) => write!(f, "{} not found", item),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for AppError {}

fn find_user(id: u32) -> Result<String, AppError> {
    if id == 0 {
        Err(AppError::InvalidInput("ID cannot be 0".to_string()))
    } else if id > 100 {
        Err(AppError::NotFound(format!("User {}", id)))
    } else {
        Ok(format!("User_{}", id))
    }
}

fn main() {
    match find_user(0) {
        Ok(user) => println!("Found: {}", user),
        Err(e) => println!("Error: {}", e),
    }

    match find_user(150) {
        Ok(user) => println!("Found: {}", user),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## From Trait สำหรับ Error Conversion

```rust
use std::io;
use std::num::ParseIntError;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Parse(e) => write!(f, "Parse error: {}", e),
            MyError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for MyError {}

// ทำให้ใช้ ? ได้
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

fn read_number_from_file(path: &str) -> Result<i32, MyError> {
    let content = std::fs::read_to_string(path)?; // io::Error -> MyError
    let number: i32 = content.trim().parse()?;    // ParseIntError -> MyError
    Ok(number)
}
```

---

## ใช้ thiserror Crate (แนะนำ)

```toml
# Cargo.toml
[dependencies]
thiserror = "1.0"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("User {0} not found")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),
}

fn process() -> Result<i32, AppError> {
    let content = std::fs::read_to_string("number.txt")?;
    let num: i32 = content.trim().parse()?;
    Ok(num)
}
```

---

## ใช้ anyhow สำหรับ Applications

```toml
# Cargo.toml
[dependencies]
anyhow = "1.0"
```

```rust,ignore
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.txt")
        .context("Failed to read config file")?;
    Ok(content)
}

fn main() -> Result<()> {
    let config = read_config()?;
    println!("Config: {}", config);
    Ok(())
}
```text

---

## Best Practices

| Library     | ใช้เมื่อ                  |
| ----------- | ------------------------- |
| Custom enum | ต้องการ match error types |
| thiserror   | เขียน library             |
| anyhow      | เขียน application         |

---

## ลองทำดู! 🎯

1. สร้าง custom error enum สำหรับ HTTP status codes
2. Implement From สำหรับ convert ระหว่าง error types
3. ลองใช้ thiserror หรือ anyhow

---

## สรุปบทที่ 9

| แนวคิด        | ใช้เมื่อ                     |
| ------------- | ---------------------------- |
| panic!        | Unrecoverable errors         |
| Result        | Recoverable errors           |
| ? operator    | Propagate errors             |
| Custom errors | ต้องการ specific error types |

👉 ต่อไป: [บทที่ 10: Generics, Traits & Lifetimes](../ch10-generics-traits/README.md)

```