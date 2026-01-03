# HashMap\<K, V\>

**HashMap** เก็บข้อมูลแบบ key-value pairs

## สร้าง HashMap

```rust,ignore
use std::collections::HashMap;

fn main() {
    // สร้าง HashMap ว่าง
    let mut scores: HashMap<String, i32> = HashMap::new();

    // เพิ่มข้อมูล
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}
```

> **หมายเหตุ:** ต้อง `use std::collections::HashMap;`

---

## สร้างจาก Iterator

```rust,ignore
use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    println!("{:?}", scores);
}
```

---

## เข้าถึงข้อมูล

```rust,ignore
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get returns Option<&V>
    let team_name = String::from("Blue");

    if let Some(score) = scores.get(&team_name) {
        println!("Blue score: {}", score);
    }

    // get_key_value returns Option<(&K, &V)>
    if let Some((key, value)) = scores.get_key_value(&team_name) {
        println!("{}: {}", key, value);
    }
}
```

---

## วนลูป

```rust,ignore
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

---

## Ownership

```rust,ignore
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name และ field_value ถูก move แล้ว!
    // println!("{}", field_name); // ❌ Error
}
```

ใช้ references หรือ clone ถ้าต้องการเก็บ:

```rust,ignore
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());

    println!("{}: {}", field_name, field_value); // ✅ OK
}
```

---

## อัปเดตค่า

### Overwrite

```rust,ignore
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrite!

    println!("{:?}", scores); // {"Blue": 25}
}
```

### Insert ถ้าไม่มี (entry)

```rust,ignore
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // ใส่ค่าเฉพาะเมื่อ key ไม่มี
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // ไม่ทำอะไร

    println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
}
```

### อัปเดตตามค่าเดิม

```rust,ignore
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // {"hello": 1, "world": 2, "wonderful": 1}
}
```

---

## Methods ที่ใช้บ่อย

```rust,ignore
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    println!("len: {}", map.len());           // 3
    println!("is_empty: {}", map.is_empty()); // false
    println!("contains a: {}", map.contains_key("a")); // true

    // remove
    let removed = map.remove("a");
    println!("removed: {:?}", removed); // Some(1)

    // keys, values
    for key in map.keys() {
        println!("key: {}", key);
    }

    for value in map.values() {
        println!("value: {}", value);
    }

    // clear
    map.clear();
    println!("after clear: {:?}", map); // {}
}
```

---

## ตัวอย่างจริง: นับความถี่

```rust,ignore
use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 2, 3, 2, 1, 3, 3, 3, 4, 5];

    let mut frequency = HashMap::new();

    for num in numbers {
        *frequency.entry(num).or_insert(0) += 1;
    }

    // หาค่าที่มากที่สุด
    let (most_common, count) = frequency
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    println!("Most common: {} ({}x)", most_common, count);
    println!("All: {:?}", frequency);
}
```

---

## ลองทำดู! 🎯

1. สร้าง phonebook ด้วย `HashMap<String, String>`
2. นับความถี่ของ characters ใน string
3. สร้าง group function ที่จัดกลุ่ม items

---

## สรุปบทที่ 8

| Collection     | ใช้เมื่อ         |
| -------------- | ---------------- |
| Vec\<T\>       | รายการลำดับ      |
| String         | ข้อความ          |
| HashMap\<K,V\> | key-value lookup |

👉 ต่อไป: [บทที่ 9: Error Handling](../ch09-error-handling/README.md)
