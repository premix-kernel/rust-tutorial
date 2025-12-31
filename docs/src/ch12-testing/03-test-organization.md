# จัดระเบียบ Tests

วิธีจัดระเบียบ tests ให้ maintain ง่าย

## #[cfg(test)]

Compile test code **เฉพาะ** เมื่อรัน `cargo test`:

```rust
// Production code
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Test code - ไม่รวมใน production build
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

**ทำไมต้องใช้?**

- ลดขนาด binary
- ไม่ compile test dependencies
- code ที่ใช้แค่ test ไม่เข้าไปใน production

---

## #[ignore] - ข้าม Tests

```rust
#[test]
fn fast_test() {
    assert!(true);
}

#[test]
#[ignore = "takes too long"]
fn slow_test() {
    std::thread::sleep(std::time::Duration::from_secs(10));
    assert!(true);
}

#[test]
#[ignore = "requires database"]
fn db_test() {
    // needs database connection
}
```

### รัน Ignored Tests

```bash
# รันเฉพาะ tests ปกติ (ข้าม ignored)
cargo test

# รันเฉพาะ ignored tests
cargo test -- --ignored

# รันทั้งหมด (รวม ignored)
cargo test -- --include-ignored
```

---

## Test Attributes รวม

```rust
#[test]
fn normal_test() {}

#[test]
#[ignore]
fn ignored_test() {}

#[test]
#[should_panic]
fn expected_panic() {
    panic!("This should happen");
}

#[test]
#[should_panic(expected = "specific message")]
fn expected_specific_panic() {
    panic!("specific message here");
}
```

---

## Controlling Test Output

```bash
# แสดง println! output (ปกติซ่อน)
cargo test -- --show-output

# รัน tests แบบ sequential (ไม่ parallel)
cargo test -- --test-threads=1

# รัน test ที่ชื่อตรง
cargo test --exact test_name

# รัน tests ที่มีคำนี้ในชื่อ
cargo test add
```

---

## Test Module Organization

### Pattern 1: Tests ใต้ function

```rust
pub fn calculate(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(5), 10);
    }
}
```

### Pattern 2: Test submodules

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn sub(a: i32, b: i32) -> i32 { a - b }

#[cfg(test)]
mod tests {
    use super::*;

    mod add_tests {
        use super::*;

        #[test]
        fn test_positive() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_negative() {
            assert_eq!(add(-2, -3), -5);
        }
    }

    mod sub_tests {
        use super::*;

        #[test]
        fn test_basic() {
            assert_eq!(sub(5, 3), 2);
        }
    }
}
```

---

## Test Helper Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper - ไม่ใช่ test
    fn create_test_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    fn assert_even(n: i32) {
        assert!(n % 2 == 0, "{} is not even", n);
    }

    #[test]
    fn test_with_helpers() {
        let data = create_test_data();
        let sum: i32 = data.iter().sum();
        assert_even(sum); // ไม่ผ่าน เพราะ 15 ไม่ใช่เลขคู่
    }
}
```

---

## Doc Tests

Comments ที่มี code examples จะถูก test:

```
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_lib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// # Panics
///
/// Panics if divisor is zero.
///
/// ```should_panic
/// my_lib::divide(10, 0);
/// ```
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

/// Code that shouldn't compile:
///
/// ```compile_fail
/// let x: i32 = "not a number";
/// ```
pub fn example() {}
````

### Run Doc Tests

```bash
# รันเฉพาะ doc tests
cargo test --doc
```

---

## Test Fixtures และ Setup

```rust
use std::fs;
use std::io::Write;

struct TestFile {
    path: String,
}

impl TestFile {
    fn new(name: &str, content: &str) -> Self {
        let path = format!("/tmp/test_{}", name);
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        TestFile { path }
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        fs::remove_file(&self.path).ok();
    }
}

#[test]
fn test_with_fixture() {
    let file = TestFile::new("example.txt", "Hello, World!");

    let content = fs::read_to_string(&file.path).unwrap();
    assert_eq!(content, "Hello, World!");
} // file ถูก cleanup อัตโนมัติ
```

---

## ลองทำดู! 🎯

1. ใช้ `#[ignore]` กับ slow tests
2. สร้าง test helper functions
3. เขียน doc tests

---

## สรุปบทที่ 12

| Attribute         | Purpose               |
| ----------------- | --------------------- |
| `#[test]`         | Mark as test          |
| `#[cfg(test)]`    | Compile only for test |
| `#[ignore]`       | Skip test             |
| `#[should_panic]` | Expect panic          |

### Cargo Commands

| Command                          | Description        |
| -------------------------------- | ------------------ |
| `cargo test`                     | Run all tests      |
| `cargo test NAME`                | Run matching tests |
| `cargo test --doc`               | Run doc tests only |
| `cargo test -- --ignored`        | Run ignored tests  |
| `cargo test -- --show-output`    | Show println!      |
| `cargo test -- --test-threads=1` | Sequential         |

### Test Organization Tips

1. ใช้ `#[cfg(test)]` สำหรับ unit tests
2. แยก integration tests ไปไว้ใน `tests/`
3. ใช้ `#[ignore]` สำหรับ slow/flaky tests
4. เขียน doc tests สำหรับ public API

👉 ต่อไป: [บทที่ 13: Iterators & Closures](../ch13-iterators-closures/README.md)
