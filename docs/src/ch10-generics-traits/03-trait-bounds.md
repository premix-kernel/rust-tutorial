# Trait Bounds

**Trait Bounds** จำกัดว่า generic type ต้องมี traits อะไรบ้าง

## Basic Syntax

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// หรือ impl syntax
fn print_summary2(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

---

## Multiple Trait Bounds

```rust,ignore
fn notify<T: Summary + Display>(item: &T) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}

// impl syntax
fn notify2(item: &(impl Summary + Display)) {
    // ...
}
```

---

## Where Clause

อ่านง่ายกว่าเมื่อมีหลาย bounds:

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
    0
}
```

---

## Return Trait

```rust
fn get_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("hello"),
    }
}
```

> **หมายเหตุ:** สามารถ return ได้แค่ type เดียว

---

## Conditionally Implement Methods

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Methods only for types with Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Larger: {}", self.x);
        } else {
            println!("Larger: {}", self.y);
        }
    }
}
```

---

## สรุป

| Syntax       | ตัวอย่าง             |
| ------------ | -------------------- |
| Single bound | `T: Display`         |
| Multiple     | `T: Display + Clone` |
| Where        | `where T: Display`   |
| Return       | `-> impl Trait`      |

👉 ต่อไป: [Lifetimes](./04-lifetimes.md)
