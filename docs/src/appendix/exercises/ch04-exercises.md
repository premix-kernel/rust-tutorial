# แบบฝึกหัด: บทที่ 4 - Control Flow

## แบบฝึกหัดที่ 1: If/Else

เขียน function `grade` ที่:

- รับ `score: i32`
- Return เกรดตามเกณฑ์:
  - 80+ = "A"
  - 70-79 = "B"
  - 60-69 = "C"
  - 50-59 = "D"
  - ต่ำกว่า 50 = "F"

<details>
<summary>ดูเฉลย</summary>

```rust
fn grade(score: i32) -> &'static str {
    if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    }
}

fn main() {
    println!("85 -> {}", grade(85)); // A
    println!("72 -> {}", grade(72)); // B
    println!("45 -> {}", grade(45)); // F
}
```

</details>

---

## แบบฝึกหัดที่ 2: Loop

เขียน loop ที่:

1. นับจาก 1 ถึง 5
2. Print แต่ละเลข
3. หยุดเมื่อถึง 5

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let mut count = 1;

    loop {
        println!("{}", count);

        if count == 5 {
            break;
        }

        count += 1;
    }
}
```

</details>

---

## แบบฝึกหัดที่ 3: While Loop

เขียนโค้ดที่:

- ใช้ while loop นับถอยหลังจาก 10 ถึง 1
- Print "Liftoff!" หลังจบ

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let mut n = 10;

    while n > 0 {
        println!("{}", n);
        n -= 1;
    }

    println!("Liftoff!");
}
```

</details>

---

## แบบฝึกหัดที่ 4: For Loop

เขียนโค้ดที่:

1. สร้าง array `[10, 20, 30, 40, 50]`
2. ใช้ for loop แสดงแต่ละค่าพร้อม index

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    for (index, value) in numbers.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
}
```

**Output:**

```
Index 0: 10
Index 1: 20
Index 2: 30
Index 3: 40
Index 4: 50
```

</details>

---

## แบบฝึกหัดที่ 5: FizzBuzz

เขียน FizzBuzz:

- เลข 1 ถึง 20
- หาร 3 ลงตัว print "Fizz"
- หาร 5 ลงตัว print "Buzz"
- หารทั้งสองลงตัว print "FizzBuzz"
- ไม่ลงตัวทั้งคู่ print เลขนั้น

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    for n in 1..=20 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

</details>

---

👉 [บทที่ 5](./ch05-exercises.md)
