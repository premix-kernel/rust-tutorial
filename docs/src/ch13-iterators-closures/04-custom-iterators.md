# Custom Iterators

สร้าง iterator ของตัวเอง โดย implement Iterator trait

## Implementation พื้นฐาน

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;  // ระบุ type ที่จะ return

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None  // หมดแล้ว
        }
    }
}

fn main() {
    let counter = Counter::new(5);

    for n in counter {
        println!("{}", n);
    }
    // Output: 1, 2, 3, 4, 5
}
```

---

## ใช้ Iterator Methods ได้ฟรี

เมื่อ implement `next()` ได้ Iterator methods ทั้งหมดมาฟรี:

```rust
fn main() {
    // sum
    let sum: u32 = Counter::new(5).sum();
    println!("Sum: {}", sum);  // 15

    // filter
    let evens: Vec<u32> = Counter::new(10)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Evens: {:?}", evens);  // [2, 4, 6, 8, 10]

    // map
    let squared: Vec<u32> = Counter::new(5)
        .map(|x| x * x)
        .collect();
    println!("Squared: {:?}", squared);  // [1, 4, 9, 16, 25]

    // chain กัน
    let result: u32 = Counter::new(10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Sum of even squares: {}", result);  // 220
}
```text

---

## ตัวอย่าง: Fibonacci Iterator

```rust
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr;

        self.curr = self.next;
        self.next = result + self.next;

        Some(result)  // Infinite iterator!
    }
}

fn main() {
    // Take first 10 Fibonacci numbers
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", fibs);
    // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    // Find first Fibonacci > 100
    let first_over_100 = Fibonacci::new()
        .find(|&x| x > 100);
    println!("First > 100: {:?}", first_over_100);  // Some(144)
}
```

---

## IntoIterator Trait

ทำให้ใช้กับ `for` loop ได้:

```rust
struct MyCollection {
    items: Vec<i32>,
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// สำหรับ reference
impl<'a> IntoIterator for &'a MyCollection {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

fn main() {
    let collection = MyCollection { items: vec![1, 2, 3] };

    // ใช้กับ for loop ได้
    for item in &collection {
        println!("{}", item);
    }

    // ยัง borrow ได้
    println!("Length: {}", collection.items.len());
}
```

---

## DoubleEndedIterator

ทำให้ `rev()` ใช้ได้:

```rust
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        Range { start, end }
    }
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = self.start;
            self.start += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Range {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            Some(self.end)
        } else {
            None
        }
    }
}

fn main() {
    // Forward
    let forward: Vec<i32> = Range::new(1, 5).collect();
    println!("Forward: {:?}", forward);  // [1, 2, 3, 4]

    // Reverse
    let backward: Vec<i32> = Range::new(1, 5).rev().collect();
    println!("Backward: {:?}", backward);  // [4, 3, 2, 1]
}
```

---

## ExactSizeIterator

ถ้ารู้ขนาด:

```rust
impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        (self.max - self.count) as usize
    }
}

fn main() {
    let counter = Counter::new(10);
    println!("Size: {}", counter.len());  // 10
}
```text

---

## ตัวอย่างจริง: Lines Iterator

```rust
struct Lines<'a> {
    remaining: &'a str,
}

impl<'a> Lines<'a> {
    fn new(s: &'a str) -> Lines<'a> {
        Lines { remaining: s }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        match self.remaining.find('\n') {
            Some(pos) => {
                let line = &self.remaining[..pos];
                self.remaining = &self.remaining[pos + 1..];
                Some(line)
            }
            None => {
                let line = self.remaining;
                self.remaining = "";
                Some(line)
            }
        }
    }
}

fn main() {
    let text = "line1\nline2\nline3";

    for line in Lines::new(text) {
        println!("Line: {}", line);
    }
}
```text

---

## ลองทำดู! 🎯

1. สร้าง iterator ที่นับถอยหลัง
2. สร้าง Fibonacci iterator และ take(20)
3. Implement IntoIterator สำหรับ custom struct

---

## สรุปบทที่ 13

| Trait                 | Purpose             | Method        |
| --------------------- | ------------------- | ------------- |
| `Iterator`            | Basic iteration     | `next()`      |
| `IntoIterator`        | Convert to iterator | `into_iter()` |
| `DoubleEndedIterator` | Reverse             | `next_back()` |
| `ExactSizeIterator`   | Known size          | `len()`       |

### Key Points

- implement `next()` → ได้ methods ฟรี
- `type Item` ระบุ type ที่ return
- `IntoIterator` ทำให้ใช้กับ `for` ได้

👉 ต่อไป: [บทที่ 14: Smart Pointers](../ch14-smart-pointers/README.md)

```