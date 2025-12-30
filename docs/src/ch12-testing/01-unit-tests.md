# Unit Tests

Rust มี testing framework ในตัว ไม่ต้องติดตั้งเพิ่ม

## โครงสร้างพื้นฐาน

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // นำเข้า items จาก parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }
}
```

---

## รัน Tests

```bash
# รัน tests ทั้งหมด
cargo test

# รัน test เฉพาะชื่อ
cargo test test_add

# รัน tests ที่มีคำนี้
cargo test add

# รัน tests ใน module
cargo test tests::
```

---

## Assertions

### assert!

```rust
#[test]
fn test_assert() {
    assert!(true);
    assert!(1 + 1 == 2);

    // Custom error message
    assert!(4 > 2, "4 should be greater than 2");

    // กับ format arguments
    let x = 5;
    assert!(x > 0, "x should be positive, got {}", x);
}
```

### assert_eq! และ assert_ne!

```rust
#[test]
fn test_equality() {
    assert_eq!(4, 2 + 2);
    assert_ne!(4, 5);

    // Custom message
    let result = 10;
    assert_eq!(result, 10, "Expected 10, got {}", result);
}
```

### debug_assert!

เฉพาะ debug builds:

```rust
fn expensive_check(x: i32) -> bool {
    // expensive computation
    x > 0
}

fn process(x: i32) {
    debug_assert!(expensive_check(x), "x must be positive");
    // ใน release build, debug_assert! ถูกลบออก
}
```

---

## ทดสอบ Panic

### #[should_panic]

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

#[test]
#[should_panic]
fn test_divide_by_zero() {
    divide(10, 0);
}

// ระบุ expected message
#[test]
#[should_panic(expected = "Cannot divide by zero")]
fn test_divide_by_zero_message() {
    divide(10, 0);
}
```

---

## ใช้ Result ใน Tests

```rust,ignore
#[test]
fn test_with_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("Math is broken"))
    }
}

// ใช้กับ ? operator
#[test]
fn test_file_operations() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("test.txt")?;
    assert!(content.contains("expected"));
    Ok(())
}
```

---

## Test Output Control

### แสดง println! output

```bash
# ปกติ output จะถูกซ่อน
cargo test

# แสดง output
cargo test -- --show-output

# หรือ
cargo test -- --nocapture
```

### ตัวอย่าง

```rust,ignore
#[test]
fn test_with_output() {
    println!("Setting up test...");
    let result = 2 + 2;
    println!("Result: {}", result);
    assert_eq!(result, 4);
    println!("Test passed!");
}
```

---

## Control Test Execution

### รันแบบ Single Thread

```bash
# ปกติ tests รันแบบ parallel
cargo test

# รันแบบ sequential
cargo test -- --test-threads=1
```

### Ignore Tests

```rust,ignore
#[test]
#[ignore]
fn expensive_test() {
    // takes a long time
    std::thread::sleep(std::time::Duration::from_secs(60));
}

// รัน ignored tests
// cargo test -- --ignored

// รันทุก tests รวม ignored
// cargo test -- --include-ignored
```

---

## Test Private Functions

ใน Rust ทดสอบ private functions ได้:

```rust
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal() {
        // ✅ เข้าถึง private function ได้
        assert_eq!(internal_adder(2, 2), 4);
    }
}
```

---

## Doc Tests

ทดสอบ code ใน documentation:

````rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

```bash
# Doc tests รันกับ cargo test
cargo test

# รันเฉพาะ doc tests
cargo test --doc
```

---

## Test Filtering

```bash
# รัน tests ที่ชื่อขึ้นต้นด้วย 'test_'
cargo test test_

# รัน tests ใน specific module
cargo test tests::unit::

# รันเฉพาะ test เดียว
cargo test tests::test_add -- --exact
```

---

## Test Organization Best Practices

```rust
// src/lib.rs

pub fn public_function() -> i32 {
    private_helper() + 1
}

fn private_helper() -> i32 {
    41
}

// Unit tests อยู่ในไฟล์เดียวกัน
#[cfg(test)]
mod tests {
    use super::*;

    mod public_api_tests {
        use super::*;

        #[test]
        fn test_public_function() {
            assert_eq!(public_function(), 42);
        }
    }

    mod internal_tests {
        use super::*;

        #[test]
        fn test_private_helper() {
            assert_eq!(private_helper(), 41);
        }
    }
}
```

---

## ลองทำดู! 🎯

1. เขียน tests สำหรับ function ที่ควร panic
2. ใช้ `#[ignore]` และลองรัน `cargo test -- --ignored`
3. เขียน doc test ใน function documentation

---

## สรุป

| Attribute/Macro   | คำอธิบาย              |
| ----------------- | --------------------- |
| `#[test]`         | Mark as test          |
| `#[cfg(test)]`    | Compile only for test |
| `#[should_panic]` | Expect panic          |
| `#[ignore]`       | Skip test             |
| `assert!`         | Check condition       |
| `assert_eq!`      | Check equality        |
| `assert_ne!`      | Check inequality      |

### Cargo Test Flags

| Flag               | คำอธิบาย          |
| ------------------ | ----------------- |
| `--show-output`    | แสดง println!     |
| `--test-threads=1` | รันแบบ sequential |
| `--ignored`        | รัน ignored tests |
| `--exact`          | Match exact name  |

👉 ต่อไป: [Integration Tests](./02-integration-tests.md)
