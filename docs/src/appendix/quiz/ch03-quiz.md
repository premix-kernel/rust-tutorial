# Quiz: บทที่ 3 - Functions

## คำถามที่ 1

Syntax ที่ถูกต้องสำหรับ function signature?

A. `function add(a: i32, b: i32) -> i32`  
B. `fn add(a: i32, b: i32) -> i32`  
C. `def add(a: i32, b: i32) -> i32`  
D. `func add(a: i32, b: i32) -> i32`

<details>
<summary>ดูเฉลย</summary>

**B. `fn add(a: i32, b: i32) -> i32`**

Rust ใช้ `fn` keyword สำหรับ function

</details>

---

## คำถามที่ 2

วิธีใดไม่ถูกต้องสำหรับ return ค่า?

A. `return x;`  
B. `x` (ไม่มี semicolon)  
C. `x;` (มี semicolon)  
D. ทั้ง A และ B ถูกต้อง

<details>
<summary>ดูเฉลย</summary>

**C. `x;` (มี semicolon)**

- `x` = expression = return ค่า
- `x;` = statement = return `()`

</details>

---

## คำถามที่ 3

function นี้ return type อะไร?

```rust,ignore
fn greet() {
    println!("Hello");
}
```

A. `String`  
B. `()`  
C. `void`  
D. `None`

<details>
<summary>ดูเฉลย</summary>

**B. `()`**

Unit type `()` คือ empty tuple หมายถึง "ไม่มีค่า return"

</details>

---

## คำถามที่ 4

Parameter ต้องระบุ type หรือไม่?

A. ต้องระบุเสมอ  
B. ไม่ต้อง compiler จะ infer  
C. ระบุเฉพาะ reference  
D. ระบุเฉพาะ generic

<details>
<summary>ดูเฉลย</summary>

**A. ต้องระบุเสมอ**

```rust,ignore
fn add(a: i32, b: i32) -> i32 { // ต้องระบุ
    a + b                        // return type ก็ต้องระบุ
}
```

</details>

---

## คำถามที่ 5

ผลลัพธ์ของ code นี้?

```rust,ignore
fn double(x: i32) -> i32 {
    x * 2
}
println!("{}", double(5));
```

A. 5  
B. 10  
C. Error  
D. 2

<details>
<summary>ดูเฉลย</summary>

**B. 10**

`double(5)` = 5 \* 2 = 10

</details>

---

👉 [Quiz บทที่ 4](./ch04-quiz.md)

```