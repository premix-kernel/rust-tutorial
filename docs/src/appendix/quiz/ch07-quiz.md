# Quiz: บทที่ 7 - Enums & Pattern Matching

## คำถาม 7.1

`Option<T>` มี variants อะไรบ้าง?

A. `Ok` และ `Err`  
B. `Some` และ `None`  
C. `True` และ `False`  
D. `Value` และ `Null`

<details>
<summary>ดูเฉลย</summary>

**B. `Some` และ `None`**

```rust,ignore
enum Option<T> {
    Some(T),
    None,
}
```

</details>

---

## คำถาม 7.2

`if let` ใช้แทนอะไร?

A. `if else`  
B. `match` ที่มี pattern เดียว  
C. `loop`  
D. `while`

<details>
<summary>ดูเฉลย</summary>

**B. `match` ที่มี pattern เดียว**

```rust,ignore
// แทนที่จะเขียน
match value {
    Some(x) => println!("{}", x),
    None => (),
}

// ใช้ if let
if let Some(x) = value {
    println!("{}", x);
}
```

</details>

---

## คำถาม 7.3

Enum variant สามารถมี data ได้ไหม?

A. ไม่ได้  
B. ได้ แต่ต้องมี type เดียว  
C. ได้ หลาย types  
D. ได้ แต่ต้องเป็น struct

<details>
<summary>ดูเฉลย</summary>

**C. ได้ หลาย types**

```rust,ignore
enum Message {
    Quit,                       // ไม่มี data
    Move { x: i32, y: i32 },   // struct-like
    Write(String),              // tuple-like
    ChangeColor(i32, i32, i32), // multiple values
}
```

</details>

---

## คำถาม 7.4

`match` ต้อง exhaustive หมายความว่าอะไร?

A. ต้องมี default case  
B. ต้อง handle ทุก variants  
C. ต้องมี guard  
D. ต้อง return ค่า

<details>
<summary>ดูเฉลย</summary>

**B. ต้อง handle ทุก variants**

ถ้า enum มี 3 variants ต้อง match ทั้ง 3 หรือใช้ `_` catch-all

</details>

---

## คำถาม 7.5

`_` ใน match หมายถึงอะไร?

A. Null value  
B. Error case  
C. Catch-all pattern  
D. Empty tuple

<details>
<summary>ดูเฉลย</summary>

**C. Catch-all pattern**

```rust,ignore
match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"), // catch all
}
```

</details>

---

👉 [Quiz บทที่ 8](./ch08-quiz.md)

```