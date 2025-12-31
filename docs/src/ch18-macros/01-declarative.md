# Declarative Macros

**Declarative macros** หรือ **macro_rules!** สร้าง code patterns ที่ซ้ำๆ

## ทำไมต้องใช้ Macros?

```rust,compile_fail
// ❌ ซ้ำซาก
let v1 = vec![1, 2, 3];
let v2 = {
    let mut temp = Vec::new();
    temp.push(1);
    temp.push(2);
    temp.push(3);
    temp
};

// ✅ ใช้ macro - สั้นกว่า
let v1 = vec![1, 2, 3];
```

---

## Syntax พื้นฐาน

```rust
macro_rules! macro_name {
    ( pattern ) => {
        expansion
    };
}
```

---

## ตัวอย่าง 1: Simple Macro

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();  // Hello!
}
```

---

## ตัวอย่าง 2: With Arguments

```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!("World");   // Hello, World!
    greet!("Rust");    // Hello, Rust!
    greet!(1 + 2);     // Hello, 3!
}
```

### Designators (ตัวจับ)

| Designator | Matches                      |
| ---------- | ---------------------------- |
| `expr`     | Expression เช่น `1 + 2`, `x` |
| `ident`    | Identifier เช่น `foo`, `x`   |
| `ty`       | Type เช่น `i32`, `String`    |
| `pat`      | Pattern เช่น `Some(x)`, `_`  |
| `block`    | Block `{ ... }`              |
| `stmt`     | Statement                    |
| `literal`  | Literal เช่น `"hello"`, `42` |
| `tt`       | Token tree (ทุกอย่าง)        |

---

## ตัวอย่าง 3: Multiple Patterns

```rust
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (sub $a:expr, $b:expr) => {
        $a - $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
}

fn main() {
    println!("{}", calculate!(add 1, 2));  // 3
    println!("{}", calculate!(sub 5, 3));  // 2
    println!("{}", calculate!(mul 4, 5));  // 20
}
```

---

## ตัวอย่าง 4: Repetition

ใช้ `$(...)*` หรือ `$(...)+` สำหรับซ้ำ:

```rust
macro_rules! vec_of_strings {
    // ( pattern ),* = zero or more, comma separated
    ($($x:expr),*) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x.to_string());
            )*
            temp
        }
    };
}

fn main() {
    let v = vec_of_strings!["a", "b", "c"];
    println!("{:?}", v);  // ["a", "b", "c"]

    let v2 = vec_of_strings![1, 2, 3];
    println!("{:?}", v2); // ["1", "2", "3"]
}
```

### Repetition Operators

| Operator | Meaning      |
| -------- | ------------ |
| `*`      | Zero or more |
| `+`      | One or more  |
| `?`      | Zero or one  |

---

## ตัวอย่าง 5: สร้าง HashMap

```rust
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 90,
    };

    println!("{:?}", scores);
}
```

---

## ตัวอย่าง 6: Debug Macro

```rust
macro_rules! debug_var {
    ($var:expr) => {
        println!("{} = {:?}", stringify!($var), $var);
    };
}

fn main() {
    let x = 42;
    let name = "Rust";
    let vec = vec![1, 2, 3];

    debug_var!(x);     // x = 42
    debug_var!(name);  // name = "Rust"
    debug_var!(vec);   // vec = [1, 2, 3]
}
```

`stringify!` แปลง expression เป็น string literal

---

## ตัวอย่าง 7: Multiple Types

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();  // Called "foo"()
    bar();  // Called "bar"()
}
```

---

## ลองทำดู! 🎯

1. สร้าง macro `min!` ที่หาค่าต่ำสุดของ 2 ค่า
2. สร้าง macro `println_all!` ที่ print หลายค่า
3. สร้าง macro ที่สร้าง struct

---

## เปรียบเทียบ Macros vs Functions

| Aspect     | Macros       | Functions |
| ---------- | ------------ | --------- |
| Evaluation | Compile-time | Runtime   |
| Types      | ไม่ต้องระบุ  | ต้องระบุ  |
| Arguments  | Variable     | Fixed     |
| Syntax     | Flexible     | Fixed     |
| Debug      | ยากกว่า      | ง่ายกว่า  |

---

## สรุป

| Pattern          | Meaning            |
| ---------------- | ------------------ |
| `$x:expr`        | Match expression   |
| `$x:ident`       | Match identifier   |
| `$($x:expr),*`   | Zero or more exprs |
| `$($x:expr),+`   | One or more exprs  |
| `$($x:expr)?`    | Optional expr      |
| `stringify!($x)` | Convert to string  |

👉 ต่อไป: [Procedural Macros](./02-procedural.md)
