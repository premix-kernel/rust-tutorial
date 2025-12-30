# Quiz: บทที่ 10 - Generics & Traits

## คำถาม 10.1

`<T>` หมายถึงอะไร?

A. Template  
B. Type parameter (generic)  
C. Tuple  
D. Trait

<details>
<summary>ดูเฉลย</summary>

**B. Type parameter (generic)**

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
```

</details>

---

## คำถาม 10.2

Lifetime `'a` ใช้ทำอะไร?

A. กำหนดเวลารัน  
B. บอก compiler ว่า reference อยู่นานแค่ไหน  
C. สร้าง async  
D. กำหนด thread

<details>
<summary>ดูเฉลย</summary>

**B. บอก compiler ว่า reference อยู่นานแค่ไหน**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

</details>

---

## คำถาม 10.3

Trait คล้ายกับอะไรในภาษาอื่น?

A. Class  
B. Interface  
C. Struct  
D. Enum

<details>
<summary>ดูเฉลย</summary>

**B. Interface**

Trait define behavior ที่ types ต้อง implement

```rust,ignore
trait Summary {
    fn summarize(&self) -> String;
}
```

</details>

---

## คำถาม 10.4

`impl Trait` ใช้ทำอะไร?

A. สร้าง trait  
B. Implement methods  
C. Return type ที่ implement trait  
D. Delete trait

<details>
<summary>ดูเฉลย</summary>

**C. Return type ที่ implement trait**

```rust
fn returns_summarizable() -> impl Summary {
    // return any type that implements Summary
}
```

</details>

---

## คำถาม 10.5

`where` clause ใช้ทำอะไร?

A. Filter data  
B. Specify trait bounds  
C. Create loops  
D. Handle errors

<details>
<summary>ดูเฉลย</summary>

**B. Specify trait bounds**

```rust,ignore
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{ ... }
```

</details>

---

👉 [Quiz บทที่ 11](./ch11-quiz.md)
