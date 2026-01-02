# Quiz: บทที่ 19 - Web Development

## คำถาม 19.1

Axum เป็น?

A. Database  
B. Web framework  
C. Template engine  
D. Testing tool

<details>
<summary>ดูเฉลย</summary>

**B. Web framework**

Axum เป็น web framework จาก Tokio team

</details>

---

## คำถาม 19.2

`Json<T>` extractor ใช้ทำอะไร?

A. Return HTML  
B. Parse JSON request body  
C. Validate form  
D. Handle cookies

<details>
<summary>ดูเฉลย</summary>

**B. Parse JSON request body**

```rust,ignore
async fn handler(Json(payload): Json<MyStruct>) { }
```

</details>

---

## คำถาม 19.3

`Path<T>` ทำอะไร?

A. Read file  
B. Extract URL path parameters  
C. Return path string  
D. Navigate to path

<details>
<summary>ดูเฉลย</summary>

**B. Extract URL path parameters**

```rust,ignore
async fn user(Path(id): Path<u32>) { }
// GET /users/123 -> id = 123
```

</details>

---

## คำถาม 19.4

`Router::new().route()` ทำอะไร?

A. สร้าง file  
B. กำหนด route และ handler  
C. เริ่ม server  
D. ปิด connection

<details>
<summary>ดูเฉลย</summary>

**B. กำหนด route และ handler**

```rust,ignore
Router::new()
    .route("/", get(handler))
    .route("/users", post(create_user))
```

</details>

---

## คำถาม 19.5

`State<T>` ใช้ทำอะไร?

A. Save to database  
B. Share application state  
C. Manage sessions  
D. Handle errors

<details>
<summary>ดูเฉลย</summary>

**B. Share application state**

```rust,ignore
async fn handler(State(db): State<DatabasePool>) { }
```

</details>

---

👉 [Quiz บทที่ 20](./ch20-quiz.md)
