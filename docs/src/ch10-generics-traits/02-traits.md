# Traits

**Traits** กำหนด behavior ที่ types ต้องมี คล้าย interfaces ในภาษาอื่น

## นิยาม Trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

---

## Implement Trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust is great"),
        author: String::from("Alice"),
        content: String::from("..."),
    };

    println!("{}", article.summarize());
}
```

---

## Default Implementation

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    title: String,
}

impl Summary for Article {} // ใช้ default

fn main() {
    let article = Article { title: String::from("News") };
    println!("{}", article.summarize()); // (Read more...)
}
```

---

## Traits as Parameters

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// หรือใช้ trait bound syntax
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

---

## Derive Attribute

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();

    println!("{:?}", p1);
    println!("Equal: {}", p1 == p2);
}
```

Common derives: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`

---

## สรุป

| แนวคิด    | Syntax                             |
| --------- | ---------------------------------- |
| Define    | `trait Name { fn method(&self); }` |
| Implement | `impl Trait for Type { ... }`      |
| Parameter | `fn func(item: &impl Trait)`       |
| Derive    | `#[derive(Debug, Clone)]`          |

👉 ต่อไป: [Trait Bounds](./03-trait-bounds.md)
