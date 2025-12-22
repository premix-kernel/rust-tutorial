# Quiz: บทที่ 17 - Unsafe Rust

## คำถาม 17.1

`unsafe` block ทำอะไรได้?

A. ทำให้โค้ดเร็วขึ้น  
B. ปิด borrow checker  
C. Dereference raw pointers  
D. ทำให้ compile ได้เสมอ

<details>
<summary>ดูเฉลย</summary>

**C. Dereference raw pointers**

Unsafe superpowers:

- Dereference raw pointers
- Call unsafe functions
- Access mutable statics
- Implement unsafe traits
- Access union fields

</details>

---

## คำถาม 17.2

`*const T` คืออะไร?

A. Constant value  
B. Raw immutable pointer  
C. Smart pointer  
D. Reference

<details>
<summary>ดูเฉลย</summary>

**B. Raw immutable pointer**

```rust
let x = 5;
let ptr: *const i32 = &x;
```

</details>

---

## คำถาม 17.3

Raw pointer ต่างจาก reference อย่างไร?

A. ไม่ต่าง  
B. Raw pointer ไม่ guaranteed valid  
C. Reference เร็วกว่า  
D. Raw pointer thread-safe

<details>
<summary>ดูเฉลย</summary>

**B. Raw pointer ไม่ guaranteed valid**

Raw pointers:

- อาจเป็น null
- อาจชี้ไปที่ invalid memory
- ไม่มี automatic cleanup

</details>

---

## คำถาม 17.4

`extern "C"` ใช้ทำอะไร?

A. Run C program  
B. Define C-compatible ABI  
C. Import extern crate  
D. Export to JavaScript

<details>
<summary>ดูเฉลย</summary>

**B. Define C-compatible ABI**

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
```

</details>

---

## คำถาม 17.5

Safe abstractions over unsafe คืออะไร?

A. ห่อ unsafe code ด้วย safe interface  
B. ลบ unsafe code  
C. แปลง unsafe เป็น async  
D. ใช้ macros แทน

<details>
<summary>ดูเฉลย</summary>

**A. ห่อ unsafe code ด้วย safe interface**

```rust
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // unsafe inside, but function is safe
    unsafe { ... }
}
```

</details>

---

👉 [Quiz บทที่ 18](./ch18-quiz.md)
