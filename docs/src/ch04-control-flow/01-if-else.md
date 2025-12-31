# if/else - เงื่อนไข

`if` ใช้ตัดสินใจว่าจะทำโค้ดส่วนไหน ตามเงื่อนไข

## Syntax พื้นฐาน

```rust
if condition {
    // ทำเมื่อ condition เป็น true
}
```

### ตัวอย่าง

```rust
fn main() {
    let number = 7;

    if number > 5 {
        println!("{} is greater than 5", number);
    }
}
```

---

## if-else

```rust
fn main() {
    let number = 3;

    if number > 5 {
        println!("Greater than 5");
    } else {
        println!("Less than or equal to 5");
    }
}
```

---

## else if

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not divisible by 4, 3, or 2");
    }
}
```

ผลลัพธ์:

```text
Divisible by 3
```
```