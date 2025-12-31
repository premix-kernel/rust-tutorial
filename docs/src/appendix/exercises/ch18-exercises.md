# แบบฝึกหัด: บทที่ 18 - Macros

## แบบฝึกหัดที่ 1: Declarative Macro

สร้าง macro `say_hello!` ที่ print "Hello!"

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

</details>

---

## แบบฝึกหัดที่ 2: Macro with Parameters

สร้าง macro `greet!` ที่รับ name

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!("World");
    greet!("Rust");
}
```

</details>

---

## แบบฝึกหัดที่ 3: Macro with Repetition

สร้าง macro `sum!` ที่รับหลายค่าและหาผลรวม

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )*
            total
        }
    };
}

fn main() {
    let result = sum!(1, 2, 3, 4, 5);
    println!("Sum: {}", result); // 15
}
```

</details>

---

## แบบฝึกหัดที่ 4: Create HashMap Macro

สร้าง macro `map!` คล้าย `vec!`

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
    };
}

fn main() {
    let scores = map! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 90,
    };

    println!("{:?}", scores);
}
```

</details>

---

## แบบฝึกหัดที่ 5: dbg! Usage

ใช้ `dbg!` macro สำหรับ debugging

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}

fn main() {
    let result = dbg!(factorial(4));
    println!("Result: {}", result);
}

// Output จะแสดง file:line และค่าของแต่ละ expression
```

</details>

---

👉 [บทที่ 19](./ch19-exercises.md)
