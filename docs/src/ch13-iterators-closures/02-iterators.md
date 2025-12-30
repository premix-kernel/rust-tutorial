# Iterators

**Iterator** ประมวลผล collection ทีละ item แบบ lazy (ไม่ทำจนกว่าจะต้องใช้)

## Iterator Trait

```rust,ignore
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // ... มี methods อื่นๆ อีกมากที่ได้มาฟรี
}
```

ทุกครั้งที่เรียก `next()`:

- `Some(item)` - ได้ item ถัดไป
- `None` - หมดแล้ว

---

## Basic Usage

```rust
fn main() {
    let v = vec![1, 2, 3];

    // สร้าง iterator
    let mut iter = v.iter();

    // เรียก next() ทีละครั้ง
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);  // หมดแล้ว
    assert_eq!(iter.next(), None);  // ยังคง None
}
```

---

## 3 วิธีสร้าง Iterator

### 1. iter() - Borrow (&T)

```rust
fn main() {
    let v = vec![1, 2, 3];

    for x in v.iter() {
        println!("{}", x);  // x is &i32
    }

    // v ยังใช้ได้
    println!("Still valid: {:?}", v);
}
```

### 2. iter_mut() - Mutable Borrow (&mut T)

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for x in v.iter_mut() {
        *x *= 2;  // แก้ไขค่าได้
    }

    println!("{:?}", v);  // [2, 4, 6]
}
```

### 3. into_iter() - Take Ownership (T)

```rust
fn main() {
    let v = vec![String::from("a"), String::from("b")];

    for s in v.into_iter() {
        println!("{}", s);  // s is String (owned)
    }

    // println!("{:?}", v);  // ❌ v ถูก move ไปแล้ว
}
```

### เปรียบเทียบ

| Method        | Returns  | Ownership      |
| ------------- | -------- | -------------- |
| `iter()`      | `&T`     | Borrow         |
| `iter_mut()`  | `&mut T` | Mutable borrow |
| `into_iter()` | `T`      | Take ownership |

---

## for Loop กับ Iterator

```rust
fn main() {
    let v = vec![1, 2, 3];

    // ทั้งสองเหมือนกัน:

    // 1. for loop (syntactic sugar)
    for x in &v {
        println!("{}", x);
    }

    // 2. Explicit iterator
    let mut iter = v.iter();
    while let Some(x) = iter.next() {
        println!("{}", x);
    }
}
```

---

## Lazy Evaluation

Iterator ไม่ทำอะไรจนกว่าจะ "consume":

```rust,ignore
fn main() {
    let v = vec![1, 2, 3];

    // ยังไม่ทำอะไร! (lazy)
    let iter = v.iter().map(|x| {
        println!("Processing {}", x);
        x * 2
    });

    println!("Iterator created, nothing printed yet...");

    // consume iterator
    let result: Vec<_> = iter.collect();
    // NOW it prints "Processing 1, 2, 3"

    println!("Result: {:?}", result);
}
```

---

## Range as Iterator

```rust
fn main() {
    // Range เป็น iterator
    for i in 0..5 {
        println!("{}", i);  // 0, 1, 2, 3, 4
    }

    // Inclusive range
    for i in 0..=5 {
        println!("{}", i);  // 0, 1, 2, 3, 4, 5
    }

    // Use with methods
    let sum: i32 = (1..=100).sum();
    println!("Sum 1-100: {}", sum);  // 5050
}
```

---

## Common Iterators

```rust
fn main() {
    // chars() - iterate characters
    for c in "hello".chars() {
        println!("{}", c);
    }

    // bytes() - iterate bytes
    for b in "hello".bytes() {
        println!("{}", b);
    }

    // lines() - iterate lines
    let text = "line1\nline2\nline3";
    for line in text.lines() {
        println!("{}", line);
    }

    // enumerate() - with index
    let v = vec!["a", "b", "c"];
    for (i, x) in v.iter().enumerate() {
        println!("{}: {}", i, x);
    }
}
```

---

## Infinite Iterators

```rust
fn main() {
    // repeat - ซ้ำค่าเดิมตลอด
    let threes: Vec<i32> = std::iter::repeat(3).take(5).collect();
    println!("{:?}", threes);  // [3, 3, 3, 3, 3]

    // cycle - วนซ้ำ
    let cycle: Vec<i32> = vec![1, 2].into_iter().cycle().take(6).collect();
    println!("{:?}", cycle);  // [1, 2, 1, 2, 1, 2]
}
```

---

## ลองทำดู! 🎯

1. ใช้ `iter()`, `iter_mut()`, `into_iter()` ดูความแตกต่าง
2. สร้าง range iterator และ sum
3. ใช้ `enumerate()` กับ vector

---

## สรุป

| Concept          | Description              |
| ---------------- | ------------------------ |
| `Iterator` trait | `next() -> Option<Item>` |
| `iter()`         | Borrow elements          |
| `iter_mut()`     | Mutable borrow           |
| `into_iter()`    | Take ownership           |
| Lazy             | ไม่ทำจนกว่า consume      |
| Infinite         | `repeat`, `cycle`        |

👉 ต่อไป: [Iterator Methods](./03-iterator-methods.md)
