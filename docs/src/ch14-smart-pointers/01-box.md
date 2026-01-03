# Box\<T\>

**Box\<T\>** เก็บข้อมูลบน heap แทน stack เป็น smart pointer ที่ง่ายที่สุด

## เมื่อไหร่ใช้ Box?

1. **Recursive types** - type ที่มีตัวเองข้างใน
2. **Large data** - ย้ายข้อมูลใหญ่ไป heap
3. **Trait objects** - dynamic dispatch
4. **Transfer ownership** โดยไม่ copy

### 📦 Smart Pointer Comparison

```text
┌─────────────────────────────────────────────────────────────────────┐
│                    Smart Pointer Cheat Sheet                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│   Pointer     │ Ownership │ Mutability │ Thread-Safe │ Use Case     │
│   ─────────── │ ───────── │ ────────── │ ─────────── │ ──────────── │
│   Box<T>      │ Single    │ Inherited  │ Yes*        │ Heap alloc   │
│   Rc<T>       │ Shared    │ Immutable  │ No          │ Multi-owner  │
│   Arc<T>      │ Shared    │ Immutable  │ Yes         │ Multi-thread │
│   RefCell<T>  │ Single    │ Runtime    │ No          │ Interior mut │
│   Mutex<T>    │ Shared    │ Exclusive  │ Yes         │ Thread mut   │
│                                                                       │
│   *Box is thread-safe if T is Send                                   │
│                                                                       │
├─────────────────────────────────────────────────────────────────────┤
│   เลือกอย่างไร?                                                      │
│                                                                       │
│   ต้องการ heap allocation อย่างเดียว? ──► Box<T>                     │
│   ต้องการ multiple owners (single thread)? ──► Rc<T>                │
│   ต้องการ multiple owners (multi thread)? ──► Arc<T>                │
│   ต้องการ mutate ผ่าน immutable ref? ──► RefCell<T>                 │
│   ต้องการ mutate ข้าม threads? ──► Arc<Mutex<T>>                    │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## การใช้งานพื้นฐาน

```rust,ignore
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // Box implements Deref ใช้เหมือน reference
    let x = *b + 1;
    println!("x = {}", x);
}
```

---

## Use Case 1: Recursive Types

### ปัญหา: ขนาดไม่แน่นอน

```rust,compile_fail
// ❌ Error: recursive type has infinite size
enum List {
    Cons(i32, List),
    Nil,
}
```

Compiler ไม่รู้ว่า `List` ใหญ่เท่าไหร่

### ทางแก้: ใช้ Box

```rust,ignore
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    // Print list
    print_list(&list);
}

fn print_list(list: &List) {
    match list {
        Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        Nil => println!("End"),
    }
}
```

**ทำไมใช้ได้?** Box มีขนาดคงที่ (pointer size) ไม่ว่าจะชี้ไปที่อะไร

```text
Stack                    Heap
┌─────────────┐         ┌─────────────┐
│ Cons(1, ────┼────────►│ Cons(2, ────┼───►...
└─────────────┘         └─────────────┘
```

---

## Use Case 2: Trait Objects

```rust,ignore
trait Draw {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing square with side {}", self.side);
    }
}

fn main() {
    // Vec ของหลาย types ที่ implement Draw
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    for shape in &shapes {
        shape.draw();
    }
}
```

---

## Use Case 3: Large Data Transfer

```rust,ignore
fn main() {
    // Large array on stack
    let big_array = [0u8; 1_000_000]; // 1MB on stack!

    // Move to heap with Box
    let boxed_array = Box::new([0u8; 1_000_000]); // 1MB on heap

    // Now ownership transfer is cheap (just move pointer)
    let another_owner = boxed_array; // Fast move!
}
```

---

## Deref Trait

Box implements `Deref` ทำให้ใช้งานเหมือน reference:

```rust,ignore
use std::ops::Deref;

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref แปลง Box<T> -> T

    // Deref coercion
    let s = Box::new(String::from("hello"));
    hello(&s); // Box<String> -> &String -> &str
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

### Deref Coercion

Rust แปลง types อัตโนมัติผ่าน Deref chain:

```rust,ignore
// Box<String> -> String -> str
// &Box<String> -> &String -> &str

fn main() {
    let b = Box::new(String::from("Rust"));

    // ทั้งหมดนี้ใช้ได้เหมือนกัน
    greet(&b);           // &Box<String>
    greet(&*b);          // &String
    greet(&(*b)[..]);    // &str
}

fn greet(s: &str) {
    println!("Hello, {}!", s);
}
```

---

## Drop Trait

Box จะ drop ทั้ง pointer และ data บน heap เมื่อออกจาก scope:

```rust,ignore
fn main() {
    {
        let b = Box::new(String::from("hello"));
        println!("{}", b);
    } // b dropped here → String on heap is freed

    println!("Box is dropped!");
}
```

### Custom Drop

```rust,ignore
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
// Output:
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!
```

### Drop Early with std::mem::drop

```rust,ignore
fn main() {
    let c = Box::new(String::from("hello"));

    println!("Before drop");
    drop(c);  // Drop early
    println!("After drop");

    // println!("{}", c); // ❌ Error: c was moved
}
```

---

## เปรียบเทียบ Box กับ Stack

| Aspect    | Stack              | Box (Heap)                |
| --------- | ------------------ | ------------------------- |
| Size      | ต้องรู้ตอน compile | ได้ตอน runtime            |
| Speed     | เร็วกว่า           | ช้ากว่าเล็กน้อย           |
| Ownership | Copy หรือ Move     | Move pointer เท่านั้น     |
| Use case  | Small, fixed-size  | Large, recursive, dynamic |

---

## ลองทำดู! 🎯

1. สร้าง Binary Tree ด้วย Box
2. Implement trait object vector
3. ลอง drop Box ก่อนออกจาก scope

---

## สรุป

| แนวคิด          | คำอธิบาย                   |
| --------------- | -------------------------- |
| `Box::new(x)`   | สร้าง Box ใส่ x บน heap    |
| `*box`          | Deref ดึงค่าออก            |
| Recursive types | ใช้ Box ให้ขนาดคงที่       |
| Trait objects   | `Box\<dyn Trait\>`         |
| Drop            | อัตโนมัติเมื่อออกจาก scope |

👉 ต่อไป: [Rc\<T\>](./02-rc.md)
