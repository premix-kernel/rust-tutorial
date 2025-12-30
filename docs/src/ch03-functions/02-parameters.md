# Parameters (พารามิเตอร์)

**Parameters** คือค่าที่ส่งเข้าไปในฟังก์ชัน เพื่อให้ฟังก์ชันทำงานกับข้อมูลที่แตกต่างกันได้

## Syntax

```rust,ignore
fn function_name(param1: Type1, param2: Type2) {
    // use param1 and param2
}
```

> **สำคัญ:** ต้องระบุ **ชนิดข้อมูล** ของทุก parameter!

---

## ตัวอย่างพื้นฐาน

```rust
fn main() {
    print_number(5);
    print_number(10);
}

fn print_number(x: i32) {
    println!("The number is: {}", x);
}
```

ผลลัพธ์:

```
The number is: 5
The number is: 10
```

---

## หลาย Parameters

```rust,ignore
fn main() {
    print_labeled_measurement(5, 'h');
    greet("Alice", 25);
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is: {}{}", value, unit);
}

fn greet(name: &str, age: u32) {
    println!("Hello, {}! You are {} years old.", name, age);
}
```

ผลลัพธ์:

```
The measurement is: 5h
Hello, Alice! You are 25 years old.
```

---

## Arguments vs Parameters

| คำศัพท์       | คำอธิบาย                   |
| ------------- | -------------------------- |
| **Parameter** | ตัวแปรในนิยามฟังก์ชัน      |
| **Argument**  | ค่าที่ส่งเข้าไปตอนเรียกใช้ |

```rust
fn add(x: i32, y: i32) { // x, y เป็น parameters
    println!("{}", x + y);
}

fn main() {
    add(5, 3); // 5, 3 เป็น arguments
}
```

---

## ชนิดข้อมูลที่ใช้บ่อย

```rust
fn main() {
    integers(10, -5);
    floats(3.14, 2.5);
    with_string("Hello");
    with_boolean(true);
}

fn integers(a: i32, b: i32) {
    println!("Integers: {} + {} = {}", a, b, a + b);
}

fn floats(a: f64, b: f64) {
    println!("Floats: {} * {} = {}", a, b, a * b);
}

fn with_string(s: &str) {
    println!("String: {}", s);
}

fn with_boolean(flag: bool) {
    println!("Boolean: {}", flag);
}
```

---

## ตัวอย่างจริง: คำนวณพื้นที่

```rust
fn main() {
    let width = 10;
    let height = 5;

    calculate_area(width, height);
}

fn calculate_area(width: u32, height: u32) {
    let area = width * height;
    println!("Width: {}", width);
    println!("Height: {}", height);
    println!("Area: {}", area);
}
```

---

## ลองทำดู! 🎯

1. สร้างฟังก์ชัน `introduce(name: &str, age: u32)` ที่พิมพ์แนะนำตัว
2. สร้างฟังก์ชัน `calculate_bmi(weight: f64, height: f64)` ที่คำนวณ BMI
3. สร้างฟังก์ชัน `print_grade(score: u32)` ที่พิมพ์เกรด

---

## สรุป

| แนวคิด              | ตัวอย่าง                 |
| ------------------- | ------------------------ |
| Single parameter    | `fn greet(name: &str)`   |
| Multiple parameters | `fn add(a: i32, b: i32)` |
| ต้องระบุชนิด        | `x: i32` ไม่ใช่แค่ `x`   |

👉 ต่อไป: [Return Values](./03-return-values.md)
