# Quiz: บทที่ 6 - Structs

## คำถามที่ 1

keyword ใดใช้สร้าง struct?

A. `class`  
B. `struct`  
C. `type`  
D. `data`

<details>
<summary>ดูเฉลย</summary>

**B. `struct`**

```rust,ignore
struct User {
    name: String,
    age: u32,
}
```

</details>

---

## คำถามที่ 2

method ต่างจาก function อย่างไร?

A. ไม่ต่างกัน  
B. method มี `self` parameter  
C. method ไม่มี return  
D. function ต้องอยู่ใน impl

<details>
<summary>ดูเฉลย</summary>

**B. method มี `self` parameter**

```rust,ignore
impl User {
    fn greet(&self) { } // method มี self
    fn new() -> Self { } // associated function ไม่มี self
}
```

</details>

---

## คำถามที่ 3

`&self` หมายถึงอะไร?

A. mutable reference to self  
B. immutable reference to self  
C. owned self  
D. static method

<details>
<summary>ดูเฉลย</summary>

**B. immutable reference to self**

- `&self` = immutable borrow
- `&mut self` = mutable borrow
- `self` = take ownership

</details>

---

## คำถามที่ 4

Tuple struct คืออะไร?

A. Struct ที่มี tuple field  
B. Struct ที่ไม่มีชื่อ field  
C. Struct ที่เป็น generic  
D. Struct ที่มี method

<details>
<summary>ดูเฉลย</summary>

**B. Struct ที่ไม่มีชื่อ field**

```rust
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);
println!("{}", red.0); // access by index
```

</details>

---

## คำถามที่ 5

`Self` ต่างจาก `self` อย่างไร?

A. ไม่ต่าง  
B. `Self` คือ type, `self` คือ instance  
C. `Self` คือ instance  
D. `self` คือ type

<details>
<summary>ดูเฉลย</summary>

**B. `Self` คือ type, `self` คือ instance**

```rust,ignore
impl User {
    fn new() -> Self { // Self = User type
        Self { name: String::new() }
    }

    fn greet(&self) { } // self = instance
}
```

</details>

---

👉 [Quiz บทที่ 7](./ch07-quiz.md)
