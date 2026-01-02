# Quiz: บทที่ 16 - Async/Await

## คำถาม 16.1

`async fn` return อะไร?

A. ค่าตรงๆ  
B. `Future`  
C. `Promise`  
D. `Task`

<details>
<summary>ดูเฉลย</summary>

**B. `Future`**

```rust,ignore
async fn fetch() -> String {
    // return impl Future<Output = String>
    String::from("data")
}
```

</details>

---

## คำถาม 16.2

`.await` ทำอะไร?

A. หยุด program  
B. รอให้ Future complete  
C. สร้าง thread  
D. Cancel operation

<details>
<summary>ดูเฉลย</summary>

**B. รอให้ Future complete**

```rust,ignore
let data = fetch().await;
```

</details>

---

## คำถาม 16.3

Tokio คืออะไร?

A. Web framework  
B. Async runtime  
C. Database  
D. Testing tool

<details>
<summary>ดูเฉลย</summary>

**B. Async runtime**

Tokio รัน async code และจัดการ tasks

</details>

---

## คำถาม 16.4

`tokio::spawn` ใช้ทำอะไร?

A. สร้าง thread  
B. สร้าง async task  
C. หยุด task  
D. Join tasks

<details>
<summary>ดูเฉลย</summary>

**B. สร้าง async task**

```rust,ignore
tokio::spawn(async {
    // background task
});
```

</details>

---

## คำถาม 16.5

`tokio::join!` ทำอะไร?

A. รวม strings  
B. รอหลาย futures พร้อมกัน  
C. สร้าง channel  
D. Lock mutex

<details>
<summary>ดูเฉลย</summary>

**B. รอหลาย futures พร้อมกัน**

```rust
let (a, b) = tokio::join!(task1(), task2());
```

</details>

---

👉 [Quiz บทที่ 17](./ch17-quiz.md)
