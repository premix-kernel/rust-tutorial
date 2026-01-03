# แบบฝึกหัด: บทที่ 12 - Testing

## แบบฝึกหัดที่ 1: Unit Test

เขียน unit test สำหรับ function `add`:

```rust,ignore
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }
}
```

</details>

---

## แบบฝึกหัดที่ 2: assert! Macros

ใช้ assert ต่างๆ:

- `assert!`
- `assert_eq!`
- `assert_ne!`

<details>
<summary>ดูเฉลย</summary>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_assertions() {
        // assert! - ตรวจสอบ boolean
        assert!(1 + 1 == 2);
        assert!(true);

        // assert_eq! - ตรวจสอบเท่ากัน
        assert_eq!(2 + 2, 4);
        assert_eq!("hello", "hello");

        // assert_ne! - ตรวจสอบไม่เท่ากัน
        assert_ne!(2 + 2, 5);
        assert_ne!("hello", "world");
    }
}
```

</details>

---

## แบบฝึกหัดที่ 3: Test Panic

เขียน test ที่ตรวจสอบว่า function panic

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero_message() {
        divide(10, 0);
    }
}
```

</details>

---

## แบบฝึกหัดที่ 4: Test Result

เขียน test ที่ return Result แทน panic

<details>
<summary>ดูเฉลย</summary>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_result() -> Result<(), String> {
        let value = "42".parse::<i32>()
            .map_err(|_| String::from("Parse failed"))?;

        if value == 42 {
            Ok(())
        } else {
            Err(String::from("Wrong value"))
        }
    }
}
```

</details>

---

## แบบฝึกหัดที่ 5: Ignore Test

เขียน test ที่จะถูกข้ามไป เว้นแต่ระบุให้รัน

<details>
<summary>ดูเฉลย</summary>

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn expensive_test() {
        // Test ที่ใช้เวลานาน
        std::thread::sleep(std::time::Duration::from_secs(10));
        assert!(true);
    }
}

// รัน ignored tests:
// cargo test -- --ignored

// รันทั้งหมด รวม ignored:
// cargo test -- --include-ignored
```

</details>

---

👉 [บทที่ 13](./ch13-exercises.md)
