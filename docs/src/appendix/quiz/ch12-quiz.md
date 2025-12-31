# Quiz: บทที่ 12 - Testing

## คำถาม 12.1

Attribute ใดใช้ mark test function?

A. `#[test]`  
B. `#[unit_test]`  
C. `@test`  
D. `test!`

<details>
<summary>ดูเฉลย</summary>

**A. `#[test]`**

```rust,ignore
#[test]
fn test_add() {
    assert_eq!(2 + 2, 4);
}
```

</details>

---

## คำถาม 12.2

`#[should_panic]` ใช้ทำอะไร?

A. ทำให้ test panic  
B. Test ผ่านถ้า function panic  
C. ข้าม test  
D. Print panic message

<details>
<summary>ดูเฉลย</summary>

**B. Test ผ่านถ้า function panic**

```rust,ignore
#[test]
#[should_panic]
fn test_divide_by_zero() {
    divide(1, 0);
}
```

</details>

---

## คำถาม 12.3

Integration tests อยู่ที่ไหน?

A. `src/` folder  
B. `tests/` folder  
C. `lib.rs`  
D. `main.rs`

<details>
<summary>ดูเฉลย</summary>

**B. `tests/` folder**

```text
project/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

</details>

---

## คำถาม 12.4

`#[ignore]` ใช้ทำอะไร?

A. Delete test  
B. Skip test by default  
C. Run only this test  
D. Mark as failed

<details>
<summary>ดูเฉลย</summary>

**B. Skip test by default**

```rust,ignore
#[test]
#[ignore]
fn expensive_test() { ... }

// รัน: cargo test -- --ignored
```

</details>

---

## คำถาม 12.5

`assert_eq!(a, b)` ต่างจาก `assert!(a == b)` อย่างไร?

A. ไม่ต่าง  
B. `assert_eq!` แสดง values เมื่อ fail  
C. `assert!` เร็วกว่า  
D. `assert_eq!` ใช้ได้เฉพาะ numbers

<details>
<summary>ดูเฉลย</summary>

**B. `assert_eq!` แสดง values เมื่อ fail**

```text
left: 5
right: 6
```

</details>

---

👉 [Quiz บทที่ 13](./ch13-quiz.md)
