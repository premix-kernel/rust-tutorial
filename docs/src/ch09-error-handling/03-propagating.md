# การส่งต่อ Error (Propagating Errors)

เมื่อ function เจอ error อาจต้องการส่งต่อให้ caller จัดการ

## วิธีดั้งเดิม: match

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // ส่งต่อ error
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // ส่งต่อ error
    }
}
```

ยาวไป! 😩

---

## ? Operator

`?` ทำให้โค้ดสั้นลง:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // ส่งต่อถ้า Err
    let mut username = String::new();
    file.read_to_string(&mut username)?; // ส่งต่อถ้า Err
    Ok(username)
}
```

---

## ? ทำงานอย่างไร

```rust,ignore
let file = File::open("file.txt")?;
```

เทียบเท่ากับ:

```rust,ignore
let file = match File::open("file.txt") {
    Ok(f) => f,
    Err(e) => return Err(e.into()), // แปลง error type ด้วย
};
```

---

## Chain ?

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

หรือสั้นกว่านี้:

```rust
use std::fs;

fn read_username_from_file() -> Result<String, String> {
    fs::read_to_string("username.txt")
        .map_err(|e| e.to_string())
}
```

---

## ? กับ Option

ใช้ `?` กับ Option ได้ด้วย:

```rust,ignore
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    println!("{:?}", last_char_of_first_line("Hello\nWorld")); // Some('o')
    println!("{:?}", last_char_of_first_line(""));             // None
}
```

---

## ? ใน main()

```rust
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let _file = File::open("hello.txt")?;
    Ok(())
}
```

---

## Error Conversion

`?` เรียก `From::from()` อัตโนมัติเพื่อแปลง error types:

```rust,ignore
use std::fs::File;
use std::io::{self, Read};

fn read_number_from_file() -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open("number.txt")?; // io::Error -> Box<dyn Error>
    let mut content = String::new();
    file.read_to_string(&mut content)?;       // io::Error -> Box<dyn Error>
    let number: i32 = content.trim().parse()?; // ParseIntError -> Box<dyn Error>
    Ok(number)
}
```

---

## ตัวอย่างจริง: API Call

```rust,ignore
use std::fs;

fn get_config_value(key: &str) -> Result<String, String> {
    let content = fs::read_to_string("config.txt")
        .map_err(|e| format!("Cannot read config: {}", e))?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 && parts[0].trim() == key {
            return Ok(parts[1].trim().to_string());
        }
    }

    Err(format!("Key '{}' not found", key))
}

fn main() {
    match get_config_value("database_url") {
        Ok(value) => println!("Found: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Early Return Pattern

```rust,ignore
fn process_data(data: &str) -> Result<String, &'static str> {
    if data.is_empty() {
        return Err("Data is empty");
    }

    let trimmed = data.trim();

    if trimmed.len() < 3 {
        return Err("Data too short");
    }

    Ok(trimmed.to_uppercase())
}

fn main() {
    println!("{:?}", process_data("  hello  ")); // Ok("HELLO")
    println!("{:?}", process_data(""));           // Err("Data is empty")
    println!("{:?}", process_data("ab"));         // Err("Data too short")
}
```

---

## ลองทำดู! 🎯

1. เขียน function ที่อ่านไฟล์และ parse เป็น JSON (ใช้ ?)
2. Chain หลาย `?` ในบรรทัดเดียว
3. สร้าง function ที่ return `Result<T, Box<dyn Error>>`

---

## สรุป

| แนวคิด       | ตัวอย่าง                     |
| ------------ | ---------------------------- |
| ? operator   | `file.read()? `              |
| Chain ?      | `File::open(path)?.read()?`  |
| ? กับ Option | `iter.next()?`               |
| main Result  | `fn main() -> Result<(), E>` |

👉 ต่อไป: [Custom Error Types](./04-custom-errors.md)
