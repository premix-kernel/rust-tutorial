# Generics

**Generics** ช่วยให้เขียนโค้ดที่ใช้ได้กับหลาย types โดยไม่ต้องเขียนซ้ำ

## ปัญหา: Code Duplication

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

โค้ดเหมือนกัน แต่ต่างแค่ type!

---

## Generic Functions

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}
```

### Syntax

```rust
fn function_name<T>(param: T) -> T {
    // ...
}

// Multiple type parameters
fn pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
```

---

## Generic Structs

```rust,compile_fail
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // ❌ Error: x และ y ต้องเป็น type เดียวกัน
    // let mixed = Point { x: 5, y: 4.0 };
}
```

### หลาย Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.0 };  // ✅ OK
}
```

---

## Generic Methods

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("x = {}", p.x());
}
```

### Methods สำหรับ Specific Type

```rust,compile_fail
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("Distance: {}", p.distance_from_origin()); // 5.0

    let p2 = Point { x: 3, y: 4 };
    // p2.distance_from_origin(); // ❌ Error: not available for Point<i32>
}
```

### Mix Type Parameters

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // Output: p3.x = 5, p3.y = c
}
```

---

## Generic Enums

Standard library ใช้ generics มากมาย:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### สร้าง Generic Enum เอง

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let a: Either<i32, String> = Either::Left(42);
    let b: Either<i32, String> = Either::Right(String::from("hello"));
}
```

---

## Monomorphization

Rust compiles generics เป็น specific types ตอน compile time:

```rust
// เราเขียน:
fn id<T>(x: T) -> T { x }

fn main() {
    id(5);
    id("hello");
}

// Compiler สร้าง:
fn id_i32(x: i32) -> i32 { x }
fn id_str(x: &str) -> &str { x }
```

**ผลลัพธ์:** zero runtime cost! เหมือนเขียนแยกเอง

---

## ตัวอย่างจริง: Container

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(item) = stack.pop() {
        println!("{}", item);
    }
    // Output: 3, 2, 1
}
```

---

## Default Type Parameters

```rust
use std::ops::Add;

// Add trait มี default type parameter
// trait Add<Rhs = Self> { ... }

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

---

## ลองทำดู! 🎯

1. สร้าง generic struct `Pair<T>` ที่มี first และ second
2. เพิ่ม method `swap()` ที่สลับ first และ second
3. สร้าง generic function ที่หาค่า min จาก slice

---

## สรุป

| แนวคิด        | Syntax                        |
| ------------- | ----------------------------- |
| Function      | `fn name<T>(arg: T)`          |
| Struct        | `struct Name<T> { field: T }` |
| Enum          | `enum Name<T> { Variant(T) }` |
| impl          | `impl<T> Name<T> { ... }`     |
| Specific impl | `impl Name<f32> { ... }`      |
| Multiple      | `<T, U, V>`                   |

### ข้อดี

- ✅ ไม่ต้องเขียนโค้ดซ้ำ
- ✅ Type-safe
- ✅ Zero runtime cost (monomorphization)

👉 ต่อไป: [Traits](./02-traits.md)
