# แบบฝึกหัด: บทที่ 9 - Error Handling

## แบบฝึกหัดที่ 1: Result

เขียน function `parse_number` ที่:

- รับ `&str`
- Return `Result<i32, String>`

<details>
<summary>ดูเฉลย</summary>

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid number", s))
}

fn main() {
    match parse_number("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("abc") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```

</details>

---

## แบบฝึกหัดที่ 2: ? Operator

แปลงโค้ดนี้ให้ใช้ `?`:

```rust
fn read_username() -> Result<String, std::io::Error> {
    let file = match std::fs::read_to_string("username.txt") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };
    Ok(file.trim().to_string())
}
```

<details>
<summary>ดูเฉลย</summary>

```rust
fn read_username() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("username.txt")?;
    Ok(content.trim().to_string())
}
```

</details>

---

## แบบฝึกหัดที่ 3: unwrap_or

แก้ไขโค้ดให้ใช้ `unwrap_or` แทน match:

```rust
let value = match some_option {
    Some(v) => v,
    None => 0,
};
```

<details>
<summary>ดูเฉลย</summary>

```rust
let value = some_option.unwrap_or(0);

// หรือใช้ unwrap_or_default() ถ้า type implement Default
let value = some_option.unwrap_or_default();
```

</details>

---

## แบบฝึกหัดที่ 4: Custom Error

สร้าง custom error `DivisionError` สำหรับการหารที่ผิดพลาด

<details>
<summary>ดูเฉลย</summary>

```rust
use std::fmt;

#[derive(Debug)]
struct DivisionError {
    message: String,
}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DivisionError {}

fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError {
            message: String::from("Cannot divide by zero"),
        })
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

</details>

---

## แบบฝึกหัดที่ 5: Chaining Results

เขียน function ที่:

1. Parse string เป็น number
2. คูณด้วย 2
3. Return Result

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>().map(|n| n * 2)
}

fn main() {
    println!("{:?}", parse_and_double("5"));  // Ok(10)
    println!("{:?}", parse_and_double("abc")); // Err(...)
}
```

</details>

---

👉 [บทที่ 10](./ch10-exercises.md)
