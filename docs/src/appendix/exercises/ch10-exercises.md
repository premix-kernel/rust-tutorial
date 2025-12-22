# แบบฝึกหัด: บทที่ 10 - Generics & Traits

## แบบฝึกหัดที่ 1: Generic Function

เขียน generic function `largest` ที่หาค่าสูงสุดจาก slice

<details>
<summary>ดูเฉลย</summary>

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest: {}", largest(&chars));
}
```

</details>

---

## แบบฝึกหัดที่ 2: Generic Struct

สร้าง generic struct `Pair<T>` ที่เก็บ 2 ค่าของ type เดียวกัน

<details>
<summary>ดูเฉลย</summary>

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

fn main() {
    let pair = Pair::new(1, 2);
    println!("{}, {}", pair.first, pair.second);

    let pair = Pair::new("hello", "world");
    println!("{}, {}", pair.first, pair.second);
}
```

</details>

---

## แบบฝึกหัดที่ 3: Trait Definition

สร้าง trait `Describable` ที่มี method `describe(&self) -> String`

<details>
<summary>ดูเฉลย</summary>

```rust
trait Describable {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} อายุ {} ปี", self.name, self.age)
    }
}

struct Product {
    name: String,
    price: f64,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("{} ราคา {} บาท", self.name, self.price)
    }
}

fn main() {
    let person = Person { name: String::from("สมชาย"), age: 30 };
    let product = Product { name: String::from("iPhone"), price: 35000.0 };

    println!("{}", person.describe());
    println!("{}", product.describe());
}
```

</details>

---

## แบบฝึกหัดที่ 4: Trait Bounds

เขียน function ที่รับ type ที่ implement ทั้ง `Display` และ `Clone`

<details>
<summary>ดูเฉลย</summary>

```rust
use std::fmt::Display;

fn print_and_clone<T: Display + Clone>(item: T) -> T {
    println!("{}", item);
    item.clone()
}

fn main() {
    let s = String::from("Hello");
    let cloned = print_and_clone(s);
    println!("Cloned: {}", cloned);
}
```

</details>

---

## แบบฝึกหัดที่ 5: Lifetime

แก้ไขให้ compile ได้:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<details>
<summary>ดูเฉลย</summary>

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long");
    let s2 = String::from("short");

    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
}
```

</details>

---

👉 [บทที่ 11](./ch11-exercises.md)
