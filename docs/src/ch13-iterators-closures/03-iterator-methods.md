# Iterator Methods

Methods ของ Iterator แบ่งเป็น 2 ประเภท: **Adapters** และ **Consumers**

## Adapters (แปลง Iterator)

Adapters รับ iterator แล้ว return iterator ใหม่:

### map - แปลงค่า

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]
}
```

### filter - เลือกเฉพาะที่ต้องการ

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();

    println!("{:?}", evens);  // [2, 4, 6]
}
```

### filter_map - filter + map รวมกัน

```rust,ignore
fn main() {
    let strings = vec!["1", "two", "3", "four", "5"];

    let numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse().ok())  // parse เฉพาะที่ได้
        .collect();

    println!("{:?}", numbers);  // [1, 3, 5]
}
```

### enumerate - เพิ่ม index

```rust,ignore
fn main() {
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
    // 0: a
    // 1: b
    // 2: c
}
```

### zip - จับคู่ 2 iterators

```rust,ignore
fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];

    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{}: {}", name, score);
    }
    // Alice: 95
    // Bob: 87
    // Charlie: 92
}
```

### take / skip

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // take: เอา n ตัวแรก
    let first_3: Vec<_> = v.iter().take(3).collect();
    println!("First 3: {:?}", first_3);  // [1, 2, 3]

    // skip: ข้าม n ตัวแรก
    let after_5: Vec<_> = v.iter().skip(5).collect();
    println!("After 5: {:?}", after_5);  // [6, 7, 8, 9, 10]

    // take_while / skip_while
    let small: Vec<_> = v.iter().take_while(|x| **x < 5).collect();
    println!("While < 5: {:?}", small);  // [1, 2, 3, 4]
}
```

### flatten - แบน nested

```rust,ignore
fn main() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];

    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("{:?}", flat);  // [1, 2, 3, 4, 5]

    // flat_map = map + flatten
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter()
        .flat_map(|s| s.chars())
        .collect();
    println!("{:?}", chars);  // ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
}
```

### chain - ต่อ 2 iterators

```rust,ignore
fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    let combined: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("{:?}", combined);  // [1, 2, 3, 4, 5, 6]
}
```

### rev - กลับลำดับ

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let reversed: Vec<_> = v.iter().rev().collect();
    println!("{:?}", reversed);  // [5, 4, 3, 2, 1]
}
```

---

## Consumers (ใช้ Iterator)

Consumers รับ iterator และ return ค่า (ไม่ใช่ iterator):

### collect - รวบรวมเป็น collection

```rust,ignore
fn main() {
    let v: Vec<i32> = (1..=5).collect();
    let set: std::collections::HashSet<_> = (1..=5).collect();

    println!("Vec: {:?}", v);    // [1, 2, 3, 4, 5]
    println!("Set: {:?}", set);  // {1, 2, 3, 4, 5}
}
```

### sum / product

```rust,ignore
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let total: i32 = numbers.iter().sum();
    println!("Sum: {}", total);  // 15

    let product: i32 = numbers.iter().product();
    println!("Product: {}", product);  // 120
}
```

### fold - รวมค่าแบบ custom

```rust,ignore
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // fold(initial, |accumulator, item| new_accumulator)
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);  // 15

    // ใช้ fold สร้าง string
    let s = numbers.iter().fold(String::new(), |acc, x| {
        if acc.is_empty() {
            x.to_string()
        } else {
            format!("{}, {}", acc, x)
        }
    });
    println!("String: {}", s);  // "1, 2, 3, 4, 5"
}
```

### reduce - เหมือน fold แต่ไม่มี initial

```rust,ignore
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let max = numbers.iter().reduce(|a, b| if a > b { a } else { b });
    println!("Max: {:?}", max);  // Some(5)
}
```

### count / min / max

```rust,ignore
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("Count: {}", v.iter().count());      // 8
    println!("Min: {:?}", v.iter().min());        // Some(1)
    println!("Max: {:?}", v.iter().max());        // Some(9)
}
```

### find / position

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // find - หาค่าแรกที่ตรง
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);  // Some(2)

    // position - หา index
    let pos = v.iter().position(|x| *x == 3);
    println!("Position of 3: {:?}", pos);  // Some(2)
}
```

### any / all

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let has_even = v.iter().any(|x| x % 2 == 0);
    println!("Has even: {}", has_even);  // true

    let all_positive = v.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);  // true
}
```

### for_each - side effects

```rust,ignore
fn main() {
    let v = vec![1, 2, 3];

    v.iter().for_each(|x| println!("Value: {}", x));
}
```

---

## Chaining (ต่อเชื่อม)

```rust,ignore
fn main() {
    let data = vec!["1", "two", "3", "four", "5", "six"];

    let result: i32 = data.iter()
        .filter_map(|s| s.parse::<i32>().ok())  // parse เฉพาะตัวเลข
        .filter(|n| n % 2 == 1)                  // เอาเฉพาะเลขคี่
        .map(|n| n * n)                          // ยกกำลังสอง
        .sum();                                   // รวม

    println!("Sum of odd squares: {}", result);  // 1 + 9 + 25 = 35
}
```

---

## ลองทำดู! 🎯

1. ใช้ `map` และ `filter` chain กัน
2. ใช้ `enumerate` และ `zip`
3. ใช้ `fold` หาค่า max เอง

---

## สรุป

### Adapters (return Iterator)

| Method          | Purpose        |
| --------------- | -------------- |
| `map`           | แปลงค่า        |
| `filter`        | เลือกค่า       |
| `enumerate`     | เพิ่ม index    |
| `zip`           | จับคู่         |
| `take` / `skip` | เอา/ข้าม n ตัว |
| `flatten`       | แบน nested     |
| `chain`         | ต่อ iterators  |
| `rev`           | กลับลำดับ      |

### Consumers (return Value)

| Method                  | Purpose            |
| ----------------------- | ------------------ |
| `collect`               | รวมเป็น collection |
| `sum` / `product`       | รวม/คูณ            |
| `fold`                  | custom aggregation |
| `count` / `min` / `max` | นับ/หาเล็ก/ใหญ่    |
| `find` / `position`     | หาค่า/ตำแหน่ง      |
| `any` / `all`           | ตรวจเงื่อนไข       |

👉 ต่อไป: [Custom Iterators](./04-custom-iterators.md)
