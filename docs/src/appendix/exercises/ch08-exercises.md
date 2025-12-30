# แบบฝึกหัด: บทที่ 8 - Collections

## แบบฝึกหัดที่ 1: Vec

สร้าง Vec และหาผลรวม:

1. สร้าง Vec ของ i32: [1, 2, 3, 4, 5]
2. เพิ่ม 6, 7, 8 เข้าไป
3. หาผลรวมทั้งหมด

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.push(6);
    numbers.push(7);
    numbers.push(8);

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum); // 36
}
```

</details>

---

## แบบฝึกหัดที่ 2: String

เขียน function ที่รับ `&str` และ return String ที่กลับหัวกลับหาง

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "Hello";
    let reversed = reverse_string(original);
    println!("{} -> {}", original, reversed); // Hello -> olleH
}
```

</details>

---

## แบบฝึกหัดที่ 3: HashMap

สร้าง HashMap ที่เก็บคะแนนนักเรียน:

- "สมชาย" = 85
- "สมหญิง" = 92
- หาคะแนนเฉลี่ย

<details>
<summary>ดูเฉลย</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("สมชาย", 85);
    scores.insert("สมหญิง", 92);

    let total: i32 = scores.values().sum();
    let count = scores.len() as i32;
    let average = total / count;

    println!("Average: {}", average); // 88
}
```

</details>

---

## แบบฝึกหัดที่ 4: Word Count

นับความถี่ของแต่ละคำใน string:

```
"the quick brown fox jumps over the lazy dog"
```

<details>
<summary>ดูเฉลย</summary>

```rust
use std::collections::HashMap;

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}
```

</details>

---

## แบบฝึกหัดที่ 5: Vec กับ Enum

สร้าง Vec ที่เก็บได้หลาย types โดยใช้ Enum:

- ตัวเลข (i32)
- ทศนิยม (f64)
- ข้อความ (String)

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
enum Value {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let data = vec![
        Value::Int(42),
        Value::Float(3.14),
        Value::Text(String::from("Hello")),
    ];

    for item in &data {
        match item {
            Value::Int(n) => println!("Int: {}", n),
            Value::Float(f) => println!("Float: {}", f),
            Value::Text(s) => println!("Text: {}", s),
        }
    }
}
```

</details>

---

👉 [บทที่ 9](./ch09-exercises.md)
