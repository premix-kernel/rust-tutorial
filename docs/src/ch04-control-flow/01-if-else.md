# if/else - เงื่อนไข

`if` ใช้ตัดสินใจว่าจะทำโค้ดส่วนไหน ตามเงื่อนไข

## Syntax พื้นฐาน

```rust,ignore
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

> **หมายเหตุ:** Rust จะหยุดเมื่อเจอเงื่อนไขแรกที่ true
> ดังนั้น 6 หาร 2 ลงตัวด้วย แต่ไม่ถูกพิมพ์

---

## Condition ต้องเป็น bool

ใน Rust condition **ต้อง**เป็น `bool`:

```rust
fn main() {
    let number = 3;

    // ❌ Error! number ไม่ใช่ bool
    // if number {
    //     println!("number is not zero");
    // }

    // ✅ ถูกต้อง
    if number != 0 {
        println!("number is not zero");
    }
}
```

---

## if ใน let (Ternary-like)

เพราะ `if` เป็น **expression** สามารถใส่ใน `let` ได้:

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("number = {}", number); // 5
}
```

> **สำคัญ:** ทุก branch ต้อง return ชนิดเดียวกัน

```rust,ignore
fn main() {
    let condition = true;

    // ❌ Error! ชนิดไม่ตรงกัน
    // let number = if condition { 5 } else { "six" };

    // ✅ ต้องเป็นชนิดเดียวกัน
    let number = if condition { 5 } else { 6 };
}
```

---

## ตัวอย่างจริง: ตรวจสอบอายุ

```rust
fn main() {
    let age = 25;

    let category = if age < 13 {
        "Child"
    } else if age < 20 {
        "Teenager"
    } else if age < 60 {
        "Adult"
    } else {
        "Senior"
    };

    println!("Age {} is {}", age, category);
}
```

---

## ตัวอย่างจริง: ตรวจสอบเกรด

```rust
fn main() {
    let score = 85;

    let grade = if score >= 80 {
        'A'
    } else if score >= 70 {
        'B'
    } else if score >= 60 {
        'C'
    } else if score >= 50 {
        'D'
    } else {
        'F'
    };

    println!("Score {} = Grade {}", score, grade);
}
```

---

## Nested if

```rust
fn main() {
    let number = 15;

    if number > 0 {
        if number % 2 == 0 {
            println!("{} is positive and even", number);
        } else {
            println!("{} is positive and odd", number);
        }
    } else {
        println!("{} is not positive", number);
    }
}
```

---

## ลองทำดู! 🎯

1. เขียนโปรแกรมตรวจสอบว่าตัวเลขเป็นบวก ลบ หรือศูนย์
2. เขียนฟังก์ชัน `max(a, b) -> i32` ที่ return ค่าที่มากกว่า
3. เขียนโปรแกรมตรวจสอบปีอธิกสุรทิน

---

## สรุป

| แนวคิด    | ตัวอย่าง                              |
| --------- | ------------------------------------- |
| if        | `if x > 0 { ... }`                    |
| if-else   | `if x > 0 { ... } else { ... }`       |
| else if   | `else if x < 0 { ... }`               |
| if ใน let | `let y = if x > 0 { 1 } else { -1 };` |

👉 ต่อไป: [Loops](./02-loops.md)
