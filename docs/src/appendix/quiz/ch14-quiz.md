# Quiz: บทที่ 14 - Smart Pointers

## คำถาม 14.1

`Box<T>` ใช้เก็บข้อมูลที่ไหน?

A. Stack  
B. Heap  
C. Static memory  
D. Register

<details>
<summary>ดูเฉลย</summary>

**B. Heap**

```rust,ignore
let b = Box::new(5); // เก็บ 5 บน heap
```text

</details>

---

## คำถาม 14.2

`Rc<T>` ต่างจาก `Box<T>` อย่างไร?

A. ไม่ต่าง  
B. `Rc` มีได้หลาย owners  
C. `Box` มีได้หลาย owners  
D. `Rc` เร็วกว่า

<details>
<summary>ดูเฉลย</summary>

**B. `Rc` มีได้หลาย owners**

`Rc` = Reference Counting ใช้เมื่อต้องการ shared ownership

</details>

---

## คำถาม 14.3

`RefCell<T>` ทำอะไรได้?

A. Multi-threading  
B. Interior mutability  
C. Garbage collection  
D. Network IO

<details>
<summary>ดูเฉลย</summary>

**B. Interior mutability**

`RefCell` ยอมให้ borrow mutably ทั้งที่เป็น immutable จากภายนอก

</details>

---

## คำถาม 14.4

`Weak<T>` ใช้แก้ปัญหาอะไร?

A. Memory leak  
B. Reference cycles  
C. Deadlock  
D. Race condition

<details>
<summary>ดูเฉลย</summary>

**B. Reference cycles**

`Weak` ไม่เพิ่ม reference count ป้องกัน cycle ที่ทำให้ memory leak

</details>

---

## คำถาม 14.5

`Rc::clone()` ทำอะไร?

A. Deep copy ข้อมูล  
B. เพิ่ม reference count  
C. สร้าง Box ใหม่  
D. ย้าย ownership

<details>
<summary>ดูเฉลย</summary>

**B. เพิ่ม reference count**

```rust,ignore
let a = Rc::new(5);
let b = Rc::clone(&a); // เพิ่ม count, ไม่ copy ข้อมูล
```

</details>

---

👉 [Quiz บทที่ 15](./ch15-quiz.md)
