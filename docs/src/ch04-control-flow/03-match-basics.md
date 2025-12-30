# Match เบื้องต้น

`match` เป็น control flow ที่ทรงพลังใน Rust ใช้เปรียบเทียบค่ากับ patterns

## Syntax พื้นฐาน

```rust,ignore
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

---

## ตัวอย่างพื้นฐาน

```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}
```

ผลลัพธ์:

```
Three
```

---

## \_ (Underscore) - Catch-all

`_` จับทุกกรณีที่ไม่ตรงกับ pattern อื่น:

```rust
fn main() {
    let number = 7;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other: {}", number),
    }
}
```

> **สำคัญ:** `match` ต้องครอบคลุมทุกกรณี (exhaustive)
> ถ้าไม่ครบ จะ compile error

---

## match กับ Return ค่า

`match` เป็น expression สามารถ return ค่าได้:

```rust
fn main() {
    let number = 2;

    let text = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };

    println!("{}", text); // two
}
```

---

## Match หลายค่า

ใช้ `|` เพื่อ match หลายค่าในครั้งเดียว:

```rust
fn main() {
    let number = 2;

    match number {
        1 | 2 | 3 => println!("One, two, or three"),
        4 | 5 | 6 => println!("Four, five, or six"),
        _ => println!("Something else"),
    }
}
```

---

## Match Range

ใช้ `..=` เพื่อ match ช่วงค่า:

```rust
fn main() {
    let number = 5;

    match number {
        1..=5 => println!("One through five"),
        6..=10 => println!("Six through ten"),
        _ => println!("Something else"),
    }
}
```

### Match กับ char

```rust
fn main() {
    let letter = 'c';

    match letter {
        'a'..='j' => println!("Early letter"),
        'k'..='z' => println!("Late letter"),
        _ => println!("Not a lowercase letter"),
    }
}
```

---

## match vs if-else

| match                     | if-else           |
| ------------------------- | ----------------- |
| ต้องครอบคลุมทุกกรณี       | ไม่จำเป็น         |
| ใช้ได้กับหลาย pattern     | ใช้ได้กับ bool    |
| อ่านง่ายกว่าเมื่อหลายกรณี | ดีสำหรับ 2-3 กรณี |

```rust
fn main() {
    let number = 3;

    // match - ชัดเจนกว่า
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };

    // if-else - ยาวกว่า
    let result2 = if number == 1 {
        "one"
    } else if number == 2 {
        "two"
    } else if number == 3 {
        "three"
    } else {
        "other"
    };
}
```

---

## ตัวอย่างจริง: วันในสัปดาห์

```rust
fn main() {
    let day = 3;

    let day_name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 | 7 => "Weekend!",
        _ => "Invalid day",
    };

    println!("Day {} is {}", day, day_name);
}
```

---

## ตัวอย่างจริง: คะแนนเป็นเกรด

```rust
fn main() {
    let score = 85;

    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?', // คะแนนไม่ถูกต้อง
    };

    println!("Score {} = Grade {}", score, grade);
}
```

---

## Multiple Lines ใน Arm

ใช้ `{ }` เมื่อต้องการหลายบรรทัด:

```rust
fn main() {
    let number = 2;

    match number {
        1 => {
            println!("Number is one");
            println!("It's the first number");
        }
        2 => {
            println!("Number is two");
            println!("It's the second number");
        }
        _ => println!("Something else"),
    }
}
```

---

## ลองทำดู! 🎯

1. เขียน match แปลงเดือน (1-12) เป็นชื่อเดือน
2. เขียน match จัดกลุ่มอายุ (เด็ก, วัยรุ่น, ผู้ใหญ่, ผู้สูงอายุ)
3. เขียน match สำหรับ rock-paper-scissors

---

## สรุปบทที่ 4

| แนวคิด  | ตัวอย่าง                               |
| ------- | -------------------------------------- |
| if/else | `if x > 0 { ... } else { ... }`        |
| loop    | `loop { break; }`                      |
| while   | `while x > 0 { ... }`                  |
| for     | `for x in 1..10 { ... }`               |
| match   | `match x { 1 => "one", _ => "other" }` |

👉 ต่อไป: [บทที่ 5: Ownership](../ch05-ownership/README.md)
