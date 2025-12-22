# แบบฝึกหัด: บทที่ 3 - Functions

## แบบฝึกหัดที่ 1: Function พื้นฐาน

เขียน function `greet` ที่:

- รับ parameter `name: &str`
- Print "Hello, {name}!"

<details>
<summary>ดูเฉลย</summary>

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("World");
    greet("Rust");
}
```

</details>

---

## แบบฝึกหัดที่ 2: Return Value

เขียน function `add` ที่:

- รับ `a: i32` และ `b: i32`
- Return ผลรวม

<details>
<summary>ดูเฉลย</summary>

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // ไม่มี semicolon = return
}

// หรือ
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
}
```

</details>

---

## แบบฝึกหัดที่ 3: Multiple Parameters

เขียน function `calculate_rectangle_area` ที่:

- รับ `width: f64` และ `height: f64`
- Return พื้นที่ (width \* height)

<details>
<summary>ดูเฉลย</summary>

```rust
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn main() {
    let area = calculate_rectangle_area(5.0, 3.0);
    println!("Area: {}", area); // 15.0
}
```

</details>

---

## แบบฝึกหัดที่ 4: Early Return

เขียน function `is_even` ที่:

- รับ `n: i32`
- Return `true` ถ้าเป็นเลขคู่, `false` ถ้าเป็นเลขคี่

<details>
<summary>ดูเฉลย</summary>

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// หรือใช้ early return
fn is_even_v2(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    false
}

fn main() {
    println!("4 is even: {}", is_even(4)); // true
    println!("7 is even: {}", is_even(7)); // false
}
```

</details>

---

## แบบฝึกหัดที่ 5: Function ที่ Return Tuple

เขียน function `min_max` ที่:

- รับ `a: i32`, `b: i32`, `c: i32`
- Return tuple `(min, max)`

<details>
<summary>ดูเฉลย</summary>

```rust
fn min_max(a: i32, b: i32, c: i32) -> (i32, i32) {
    let min = a.min(b).min(c);
    let max = a.max(b).max(c);
    (min, max)
}

fn main() {
    let (min, max) = min_max(5, 2, 8);
    println!("min: {}, max: {}", min, max); // min: 2, max: 8
}
```

</details>

---

👉 [บทที่ 4](./ch04-exercises.md)
