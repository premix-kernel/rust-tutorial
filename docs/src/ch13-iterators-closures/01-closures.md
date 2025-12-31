# Closures

**Closures** คือ anonymous functions ที่สามารถ capture ค่าจาก environment รอบข้าง

## Syntax พื้นฐาน

```rust
fn main() {
    // Closure พื้นฐาน
    let add_one = |x| x + 1;
    println!("{}", add_one(5));  // 6

    // กับ type annotations
    let add_two = |x: i32| -> i32 { x + 2 };
    println!("{}", add_two(5));  // 7

    // หลาย parameters
    let add = |a, b| a + b;
    println!("{}", add(2, 3));  // 5

    // ไม่มี parameters
    let say_hi = || println!("Hi!");
    say_hi();

    // Multi-line closure
    let calculate = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        sum + product
    };
    println!("{}", calculate(3, 4));  // 7 + 12 = 19
}
```

---

## Closure vs Function

```rust
fn add_one_fn(x: i32) -> i32 { x + 1 }
let add_one_closure = |x: i32| x + 1;

// ทั้งสองใช้เหมือนกัน
println!("{}", add_one_fn(5));       // 6
println!("{}", add_one_closure(5));  // 6
```

| Aspect  | Function     | Closure             |
| ------- | ------------ | ------------------- |
| Capture | ❌ ไม่ได้    | ✅ ได้              |
| Syntax  | `fn name()`  | `\|x\| x + 1`       |
| Inline  | ❌           | ✅                  |
| Generic | ❌ (ต้องแยก) | ✅ (type inference) |

---

## Capture Environment

Closures สามารถ "จับ" ค่าจากภายนอกได้:

```rust
fn main() {
    let x = 4;
    let multiplier = 3;

    // capture x และ multiplier
    let calculate = |y| x + y * multiplier;

    println!("{}", calculate(2));  // 4 + 2*3 = 10

    // x และ multiplier ยังใช้ได้ (ถูก borrow)
    println!("x = {}, multiplier = {}", x, multiplier);
}
```

---

## 3 Capture Modes

Rust มี 3 วิธีที่ closure capture ค่า:

### 1. Fn - Borrow (&T)

```rust
fn main() {
    let list = vec![1, 2, 3];

    // Fn: borrow list
    let print_list = || println!("{:?}", list);

    print_list();
    print_list();  // เรียกได้หลายครั้ง

    println!("Still valid: {:?}", list);  // ยังใช้ได้
}
```

### 2. FnMut - Mutable Borrow (&mut T)

```rust
fn main() {
    let mut count = 0;

    // FnMut: mutable borrow count
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };

    increment();  // Count: 1
    increment();  // Count: 2
    increment();  // Count: 3

    println!("Final: {}", count);  // 3
}
```

### 3. FnOnce - Move Ownership (T)

```rust,compile_fail
fn main() {
    let message = String::from("Hello");

    // FnOnce: move ownership
    let consume = move || {
        println!("{}", message);
        // message ถูก drop เมื่อ closure จบ
    };

    consume();
    // consume();  // ❌ Error: ใช้ได้แค่ครั้งเดียว

    // println!("{}", message);  // ❌ message ถูก move ไปแล้ว
}
```

---

## move Keyword

บังคับให้ closure เป็นเจ้าของค่า:

```rust,compile_fail
fn main() {
    let name = String::from("Alice");

    // Without move: borrow
    let greet1 = || println!("Hello, {}!", name);
    greet1();
    println!("Name still valid: {}", name);

    // With move: take ownership
    let greet2 = move || println!("Hello, {}!", name);
    greet2();
    // println!("{}", name);  // ❌ name moved
}
```

### ใช้ move กับ Threads

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // ต้อง move เพราะ thread อาจอยู่นานกว่า main
    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);
    });

    handle.join().unwrap();
}
```

---

## Closures as Parameters

### ใช้ Trait Bounds

```rust
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_twice<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}

fn apply_many<F>(f: F, times: u32)
where
    F: Fn(),
{
    for _ in 0..times {
        f();
    }
}

fn main() {
    apply(|| println!("Called once!"));
    apply_twice(|| println!("Called!"));
    apply_many(|| println!("Hello!"), 3);
}
```

### Trait Hierarchy

```
FnOnce  (ทุก closure เป็น FnOnce)
  ↓
FnMut   (ถ้าไม่ move ownership)
  ↓
Fn      (ถ้าไม่ mutate)
```

---

## Closures as Return Values

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {
    let add_5 = make_adder(5);
    let add_10 = make_adder(10);

    println!("{}", add_5(3));   // 8
    println!("{}", add_10(3));  // 13
}
```text

---

## ตัวอย่างจริง: Map และ Filter

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // map กับ closure
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);  // [2, 4, 6, 8, 10]

    // filter กับ closure
    let evens: Vec<&i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("Evens: {:?}", evens);  // [2, 4]

    // Combined
    let sum_of_squares: i32 = numbers
        .iter()
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Sum of even squares: {}", sum_of_squares);  // 4 + 16 = 20
}
```

---

## ลองทำดู! 🎯

1. สร้าง closure ที่ capture mutable variable และเพิ่มค่า
2. สร้าง function ที่รับ closure เป็น parameter
3. ใช้ `move` กับ `thread::spawn`

---

## สรุป

| Trait    | Capture | เรียกได้  | ใช้เมื่อ       |
| -------- | ------- | --------- | -------------- |
| `Fn`     | &T      | หลายครั้ง | ไม่ mutate     |
| `FnMut`  | &mut T  | หลายครั้ง | mutate ได้     |
| `FnOnce` | T       | 1 ครั้ง   | move ownership |

| Keyword     | Effect                |
| ----------- | --------------------- |
| `move`      | บังคับ take ownership |
| `\|x\|`     | closure parameters    |
| `impl Fn()` | return closure        |

👉 ต่อไป: [Iterators](./02-iterators.md)

```