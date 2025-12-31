# แบบฝึกหัด: บทที่ 2 - Variables & Data Types

## แบบฝึกหัดที่ 1: Mutability

แก้ไขโค้ดนี้ให้ทำงานได้:

```rust
fn main() {
    let x = 5;
    x = 10;
    println!("{}", x);
}
```

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let mut x = 5;  // เพิ่ม mut
    x = 10;
    println!("{}", x);
}
```

</details>

---

## แบบฝึกหัดที่ 2: Type Annotation

เติม type ให้ถูกต้อง:

```rust
fn main() {
    let a: ____ = 42;
    let b: ____ = 3.14;
    let c: ____ = true;
    let d: ____ = 'A';
    let e: ____ = "Hello";
}
```

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'A';
    let e: &str = "Hello";
}
```

</details>

---

## แบบฝึกหัดที่ 3: Shadowing

ผลลัพธ์ของโค้ดนี้คืออะไร?

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {}", x);
    }
    println!("outer: {}", x);
}
```

<details>
<summary>ดูเฉลย</summary>

```text
inner: 12
outer: 6
```

**อธิบาย:**

- `x = 5`
- `x = 5 + 1 = 6` (shadowing)
- ใน block: `x = 6 * 2 = 12` (shadowing เฉพาะใน block)
- เมื่อออกจาก block: `x` กลับมาเป็น 6

</details>

---

## แบบฝึกหัดที่ 4: Tuple และ Array

สร้างโค้ดที่:

1. สร้าง tuple `(i32, f64, bool)` ค่า `(10, 3.14, true)`
2. Destructure ออกมาเป็น 3 ตัวแปร
3. สร้าง array ของ i32 ขนาด 5 ค่า [1, 2, 3, 4, 5]
4. Print ค่าแรกและค่าสุดท้าย

<details>
<summary>ดูเฉลย</summary>

```rust
fn main() {
    // Tuple
    let tup: (i32, f64, bool) = (10, 3.14, true);
    let (a, b, c) = tup;
    println!("a={}, b={}, c={}", a, b, c);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first: {}, last: {}", arr[0], arr[4]);
}
```

</details>

---

## แบบฝึกหัดที่ 5: Constants

ประกาศ constant ที่:

1. ชื่อ `MAX_USERS`
2. Type `u32`
3. ค่า 1000

<details>
<summary>ดูเฉลย</summary>

```rust
const MAX_USERS: u32 = 1000;

fn main() {
    println!("Max users: {}", MAX_USERS);
}
```

**หมายเหตุ:**

- `const` ต้องระบุ type เสมอ
- ใช้ SCREAMING_SNAKE_CASE
- ต้องกำหนดค่าตอน compile time

</details>

---

👉 [บทที่ 3](./ch03-exercises.md)
