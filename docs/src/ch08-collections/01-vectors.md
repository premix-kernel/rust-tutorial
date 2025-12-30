# Vec\<T\> - Vector

**Vector** เป็น dynamic array ที่ขยายขนาดได้

## สร้าง Vector

```rust,ignore
fn main() {
    // สร้าง vector ว่าง
    let v1: Vec<i32> = Vec::new();

    // สร้างด้วย vec! macro
    let v2 = vec![1, 2, 3];

    println!("{:?}", v1); // []
    println!("{:?}", v2); // [1, 2, 3]
}
```

---

## เพิ่มข้อมูล

```rust,ignore
fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v); // [1, 2, 3]
}
```

---

## เข้าถึงข้อมูล

### แบบที่ 1: Indexing

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("Third element: {}", third);

    // ⚠️ panic ถ้า index ไม่มี!
    // let hundred = &v[100]; // 💥 panic!
}
```

### แบบที่ 2: get() (ปลอดภัยกว่า)

```rust,ignore
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    match v.get(2) {
        Some(value) => println!("Third: {}", value),
        None => println!("No element"),
    }

    // v.get(100) returns None (ไม่ panic)
    if let Some(value) = v.get(100) {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }
}
```

---

## วนลูป

### Immutable iteration

```rust
fn main() {
    let v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }

    println!("v still accessible: {:?}", v);
}
```

### Mutable iteration

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for i in &mut v {
        *i *= 2; // dereference with *
    }

    println!("{:?}", v); // [2, 4, 6]
}
```

---

## Methods ที่ใช้บ่อย

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    println!("len: {}", v.len());       // 5
    println!("is_empty: {}", v.is_empty()); // false
    println!("first: {:?}", v.first()); // Some(1)
    println!("last: {:?}", v.last());   // Some(5)

    // pop - ลบและ return ตัวสุดท้าย
    let last = v.pop();
    println!("popped: {:?}", last); // Some(5)
    println!("v: {:?}", v); // [1, 2, 3, 4]

    // insert - แทรกที่ตำแหน่ง
    v.insert(1, 10);
    println!("after insert: {:?}", v); // [1, 10, 2, 3, 4]

    // remove - ลบที่ตำแหน่ง
    let removed = v.remove(1);
    println!("removed: {}", removed); // 10
    println!("after remove: {:?}", v); // [1, 2, 3, 4]

    // contains
    println!("contains 3: {}", v.contains(&3)); // true
}
```

---

## Vector กับ Enum

เก็บหลาย types ด้วย enum:

```rust,ignore
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
```

---

## Ownership และ Vector

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // borrow

    // v.push(6); // ❌ Error! cannot borrow v as mutable

    println!("First: {}", first);
}
```

> **ทำไม?** เพราะ push อาจทำให้ vector ย้ายที่ในเมมโมรี่
> ทำให้ reference เดิมไม่ valid

---

## ลองทำดู! 🎯

1. สร้าง vector ของเลข 1-10 แล้วคำนวณผลรวม
2. สร้าง function ที่รับ `&mut Vec<i32>` และกรองเอาเฉพาะเลขคู่
3. ใช้ enum เก็บหลาย types ใน vector

---

## สรุป

| Method        | คำอธิบาย              |
| ------------- | --------------------- |
| `Vec::new()`  | สร้าง vector ว่าง     |
| `vec![...]`   | สร้าง vector พร้อมค่า |
| `push(x)`     | เพิ่มท้าย             |
| `pop()`       | ลบท้าย                |
| `v[i]`        | เข้าถึง (panic ได้)   |
| `v.get(i)`    | เข้าถึง (ปลอดภัย)     |
| `for x in &v` | iterate               |

👉 ต่อไป: [String](./02-strings.md)
