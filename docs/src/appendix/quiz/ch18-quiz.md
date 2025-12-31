# Quiz: บทที่ 18 - Macros

## คำถาม 18.1

`macro_rules!` ใช้สร้าง macro แบบไหน?

A. Procedural  
B. Declarative  
C. Attribute  
D. Derive

<details>
<summary>ดูเฉลย</summary>

**B. Declarative**

```rust,ignore
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}
```

</details>

---

## คำถาม 18.2

`$x:expr` หมายถึง?

A. Expression ที่ชื่อ x  
B. Macro parameter ที่ match expression  
C. Variable x  
D. Type x

<details>
<summary>ดูเฉลย</summary>

**B. Macro parameter ที่ match expression**

Fragment specifiers:

- `expr` = expression
- `ident` = identifier
- `ty` = type
- `stmt` = statement

</details>

---

## คำถาม 18.3

`$(...)*` หมายถึงอะไร?

A. Pointer  
B. Zero or more repetitions  
C. Multiplication  
D. Reference

<details>
<summary>ดูเฉลย</summary>

**B. Zero or more repetitions**

```rust,ignore
macro_rules! vec_like {
    ($($x:expr),*) => { ... };
}
```

</details>

---

## คำถาม 18.4

Procedural macros ต่างจาก declarative อย่างไร?

A. ไม่ต่าง  
B. Procedural ทำงานกับ token stream  
C. Declarative เร็วกว่า  
D. Procedural อยู่ใน file เดียว

<details>
<summary>ดูเฉลย</summary>

**B. Procedural ทำงานกับ token stream**

Procedural macros:

- รับ TokenStream
- Return TokenStream
- ต้องอยู่ใน crate แยก

</details>

---

## คำถาม 18.5

`#[derive(Debug)]` คือ macro ประเภทไหน?

A. Declarative  
B. Function-like  
C. Derive macro  
D. Attribute macro

<details>
<summary>ดูเฉลย</summary>

**C. Derive macro**

Derive macros auto-implement traits

</details>

---

👉 [Quiz บทที่ 19](./ch19-quiz.md)

```