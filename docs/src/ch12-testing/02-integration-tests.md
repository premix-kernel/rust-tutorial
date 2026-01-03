# Integration Tests

**Integration tests** ทดสอบ library จากมุมมองของ user ภายนอก

## Unit Tests vs Integration Tests

| Aspect   | Unit Tests         | Integration Tests     |
| -------- | ------------------ | --------------------- |
| Location | `src/` (ใน module) | `tests/` (แยกต่างหาก) |
| Access   | Private items ได้  | Public API เท่านั้น   |
| Purpose  | Test ทีละ unit     | Test การทำงานรวม      |
| Speed    | เร็วกว่า           | ช้ากว่า               |

---

## โครงสร้างโปรเจกต์

```text
my_library/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs        ← shared test utilities
    ├── integration_test.rs
    └── another_test.rs
```

---

## สร้าง Integration Test

### src/lib.rs

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { value: 0 }
    }

    pub fn add(&mut self, n: i32) {
        self.value += n;
    }

    pub fn result(&self) -> i32 {
        self.value
    }
}
```

### tests/integration_test.rs

```rust,ignore
use my_library::{add, multiply, Calculator};

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2, 3), 6);
    assert_eq!(multiply(-2, 3), -6);
    assert_eq!(multiply(0, 100), 0);
}

#[test]
fn test_calculator_workflow() {
    let mut calc = Calculator::new();

    calc.add(5);
    calc.add(10);
    calc.add(-3);

    assert_eq!(calc.result(), 12);
}
```

---

## Shared Test Code

### tests/common/mod.rs

```rust,ignore
use my_library::Calculator;

pub fn setup_calculator_with_value(value: i32) -> Calculator {
    let mut calc = Calculator::new();
    calc.add(value);
    calc
}

pub fn assert_close(a: f64, b: f64, epsilon: f64) {
    assert!((a - b).abs() < epsilon,
            "Expected {} to be close to {}", a, b);
}

pub struct TestContext {
    pub name: String,
    pub created_at: std::time::Instant,
}

impl TestContext {
    pub fn new(name: &str) -> Self {
        println!("Setting up test: {}", name);
        TestContext {
            name: name.to_string(),
            created_at: std::time::Instant::now(),
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("Tearing down test: {} (took {:?})",
                 self.name,
                 self.created_at.elapsed());
    }
}
```

### ใช้ Shared Code

```rust,ignore
// tests/integration_test.rs
mod common;

use my_library::Calculator;

#[test]
fn test_with_setup() {
    let _ctx = common::TestContext::new("calculator_test");

    let calc = common::setup_calculator_with_value(100);
    assert_eq!(calc.result(), 100);
}
```

---

## รัน Integration Tests

```bash
# รันทุก tests (unit + integration)
cargo test

# รันเฉพาะ integration tests
cargo test --test integration_test

# รันเฉพาะ test เดียว
cargo test --test integration_test test_add

# รันทุก integration test files
cargo test --test '*'
```

---

## Binary Crates

**สำคัญ:** Integration tests ใช้ได้กับ **library crates** เท่านั้น

ถ้ามีแค่ `main.rs` (binary crate):

```rust,ignore
// src/main.rs
fn main() {
    let result = add(2, 3);
    println!("{}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**ทางแก้:** แยก logic ไปไว้ใน `lib.rs`:

```rust,ignore
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// src/main.rs
use my_crate::add;

fn main() {
    println!("{}", add(2, 3));
}
```

---

## หลาย Test Files

```rust,ignore
// tests/math_tests.rs
use my_library::{add, multiply};

#[test]
fn test_math_operations() {
    assert_eq!(add(multiply(2, 3), 4), 10);
}

// tests/calculator_tests.rs
use my_library::Calculator;

#[test]
fn test_calculator_chain() {
    let mut calc = Calculator::new();
    calc.add(1);
    calc.add(2);
    calc.add(3);
    assert_eq!(calc.result(), 6);
}
```

---

## Async Integration Tests

```rust,ignore
// tests/async_test.rs

#[tokio::test]
async fn test_async_function() {
    let result = my_library::async_fetch().await;
    assert!(result.is_ok());
}
```

---

## ลองทำดู! 🎯

1. สร้าง `tests/` folder และ integration test
2. สร้าง shared utilities ใน `tests/common/mod.rs`
3. รันเฉพาะ integration tests ด้วย `--test`

---

## สรุป

| Command                  | Description            |
| ------------------------ | ---------------------- |
| `cargo test`             | Run all tests          |
| `cargo test --test NAME` | Run specific test file |
| `cargo test --lib`       | Run only unit tests    |
| `cargo test --doc`       | Run only doc tests     |

### File Structure

```text
tests/
├── common/mod.rs     ← shared code (NOT a test)
├── test_a.rs         ← first test file
└── test_b.rs         ← second test file
```

👉 ต่อไป: [จัดระเบียบ Tests](./03-test-organization.md)
