# Quiz: บทที่ 13 - Iterators & Closures

## คำถาม 13.1

Closure คืออะไร?

A. Function ที่ปิด scope  
B. Anonymous function ที่ capture environment  
C. Loop ที่หยุดเอง  
D. Error handler

<details>
<summary>ดูเฉลย</summary>

**B. Anonymous function ที่ capture environment**

```rust,ignore
let x = 10;
let add_x = |y| y + x; // capture x
```text

</details>

---

## คำถาม 13.2

`.collect()` ทำอะไร?

A. รวม iterators  
B. แปลง iterator เป็น collection  
C. นับ items  
D. กรอง items

<details>
<summary>ดูเฉลย</summary>

**B. แปลง iterator เป็น collection**

```rust,ignore
let v: Vec<i32> = (1..5).collect();
```text

</details>

---

## คำถาม 13.3

`Fn`, `FnMut`, `FnOnce` ต่างกันอย่างไร?

A. ไม่ต่าง  
B. วิธี capture environment  
C. Return type  
D. Parameter count

<details>
<summary>ดูเฉลย</summary>

**B. วิธี capture environment**

- `Fn` - borrow (&)
- `FnMut` - borrow mutably (&mut)
- `FnOnce` - take ownership

</details>

---

## คำถาม 13.4

`.map()` ทำอะไร?

A. Filter elements  
B. Transform each element  
C. Sum all elements  
D. Find first match

<details>
<summary>ดูเฉลย</summary>

**B. Transform each element**

```rust,ignore
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect(); // [2, 4, 6]
```text

</details>

---

## คำถาม 13.5

Iterators ใน Rust เป็น lazy หมายความว่า?

A. ทำงานช้า  
B. ไม่ทำงานจนกว่าจะ consume  
C. ใช้ memory เยอะ  
D. ต้องใช้ async

<details>
<summary>ดูเฉลย</summary>

**B. ไม่ทำงานจนกว่าจะ consume**

```rust
let iter = (1..1000).map(|x| x * 2);
// ยังไม่ทำอะไรเลย จนกว่าจะ .collect() หรือ .for_each()
```

</details>

---

👉 [Quiz บทที่ 14](./ch14-quiz.md)
