# แบบฝึกหัด: บทที่ 13 - Iterators & Closures

## แบบฝึกหัดที่ 1: Closure พื้นฐาน

สร้าง closure ที่บวกเลข 2 ตัว

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let add = |a, b| a + b;

    println!("5 + 3 = {}", add(5, 3));

    // หรือระบุ type
    let add_typed = |a: i32, b: i32| -> i32 { a + b };
    println!("5 + 3 = {}", add_typed(5, 3));
}
```

</details>

---

## แบบฝึกหัดที่ 2: Closure ที่ capture

สร้าง closure ที่ capture ตัวแปรจากภายนอก

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn main() {
    let multiplier = 10;

    // Capture by reference
    let multiply = |x| x * multiplier;

    println!("5 * {} = {}", multiplier, multiply(5));

    // Capture by value (move)
    let greeting = String::from("Hello");
    let greet = move |name| format!("{}, {}!", greeting, name);

    println!("{}", greet("World"));
    // println!("{}", greeting); // Error: greeting moved
}
```

</details>

---

## แบบฝึกหัดที่ 3: Iterator Methods

ใช้ `map`, `filter`, `collect`:

1. สร้าง Vec [1, 2, 3, 4, 5]
2. กรองเอาเฉพาะเลขคู่
3. คูณแต่ละตัวด้วย 2
4. Collect เป็น Vec

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    println!("{:?}", result); // [4, 8]
}
```

</details>

---

## แบบฝึกหัดที่ 4: fold

ใช้ `fold` หาผลรวมและผลคูณ

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // ผลรวม
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum); // 15

    // ผลคูณ
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product); // 120
}
```

</details>

---

## แบบฝึกหัดที่ 5: Custom Iterator

สร้าง iterator ที่นับเลข

<details>
<summary>ดูเฉลย</summary>

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Self { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);

    for num in counter {
        println!("{}", num);
    }
    // 1, 2, 3, 4, 5
}
```

</details>

---

👉 [บทที่ 14](./ch14-exercises.md)
