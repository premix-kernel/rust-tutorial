# Quiz: บทที่ 15 - Concurrency

## คำถาม 15.1

`thread::spawn` ทำอะไร?

A. หยุด thread  
B. สร้าง thread ใหม่  
C. รวม threads  
D. Lock thread

<details>
<summary>ดูเฉลย</summary>

**B. สร้าง thread ใหม่**

```rust
let handle = thread::spawn(|| {
    // code ที่รันใน thread ใหม่
});
```

</details>

---

## คำถาม 15.2

`Mutex` ใช้ทำอะไร?

A. สร้าง thread  
B. ป้องกัน data race  
C. ส่ง message  
D. Copy data

<details>
<summary>ดูเฉลย</summary>

**B. ป้องกัน data race**

Mutex = Mutual Exclusion ยอมให้เข้าถึงได้ทีละ thread

</details>

---

## คำถาม 15.3

`Arc<T>` ต่างจาก `Rc<T>` อย่างไร?

A. ไม่ต่าง  
B. `Arc` thread-safe  
C. `Rc` thread-safe  
D. `Arc` เร็วกว่า

<details>
<summary>ดูเฉลย</summary>

**B. `Arc` thread-safe**

- `Rc` = single-threaded
- `Arc` = Atomic Reference Counting (thread-safe)

</details>

---

## คำถาม 15.4

`mpsc::channel()` สร้างอะไร?

A. Thread  
B. Sender และ Receiver  
C. Mutex  
D. File

<details>
<summary>ดูเฉลย</summary>

**B. Sender และ Receiver**

```rust
let (tx, rx) = mpsc::channel();
// tx = transmitter/sender
// rx = receiver
```

</details>

---

## คำถาม 15.5

`move` closure ใช้เมื่อไหร่?

A. เมื่อต้องการ copy  
B. เมื่อต้องการ move ownership เข้า closure  
C. เมื่อต้องการ reference  
D. เมื่อต้องการ print

<details>
<summary>ดูเฉลย</summary>

**B. เมื่อต้องการ move ownership เข้า closure**

```rust
let data = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", data); // data moved here
});
```

</details>

---

👉 [Quiz บทที่ 16](./ch16-quiz.md)
