# Quiz: บทที่ 9 - Error Handling

## คำถาม 9.1

`Result<T, E>` มี variants อะไร?

A. `Some` และ `None`  
B. `Ok` และ `Err`  
C. `Success` และ `Failure`  
D. `Value` และ `Error`

<details>
<summary>ดูเฉลย</summary>

**B. `Ok` และ `Err`**

```rust,ignore
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

</details>

---

## คำถาม 9.2

`?` operator ทำอะไร?

A. Return `None`  
B. Unwrap หรือ return error  
C. Panic  
D. Print error

<details>
<summary>ดูเฉลย</summary>

**B. Unwrap หรือ return error**

```rust,ignore
fn read_file() -> Result<String, Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}
```

ถ้า error จะ return Err ออกจาก function ทันที

</details>

---

## คำถาม 9.3

`panic!` ใช้เมื่อไหร่?

A. ทุก error  
B. Unrecoverable errors  
C. Network errors  
D. User input errors

<details>
<summary>ดูเฉลย</summary>

**B. Unrecoverable errors**

`panic!` ใช้เมื่อโปรแกรมไม่สามารถดำเนินต่อได้อย่างปลอดภัย

</details>

---

## คำถาม 9.4

`unwrap()` ต่างจาก `expect()` อย่างไร?

A. ไม่ต่าง  
B. `expect` มี custom message  
C. `unwrap` ไม่ panic  
D. `expect` return Option

<details>
<summary>ดูเฉลย</summary>

**B. `expect` มี custom message**

```rust,ignore
let x = some_option.unwrap();       // generic message
let x = some_option.expect("msg"); // custom message
```

</details>

---

## คำถาม 9.5

`unwrap_or()` ทำอะไร?

A. Panic ถ้า None  
B. Return ค่า default ถ้า None  
C. Return Result  
D. แปลงเป็น Option

<details>
<summary>ดูเฉลย</summary>

**B. Return ค่า default ถ้า None**

```rust,ignore
let x = some_option.unwrap_or(0); // ใช้ 0 ถ้า None
```

</details>

---

👉 [Quiz บทที่ 10](./ch10-quiz.md)
