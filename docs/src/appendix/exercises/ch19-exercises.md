# แบบฝึกหัด: บทที่ 19 - Web Development

## แบบฝึกหัดที่ 1: Hello World Server

สร้าง Axum server ที่ return "Hello, World!"

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use axum::{Router, routing::get};

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

</details>

---

## แบบฝึกหัดที่ 2: Path Parameters

สร้าง route `/hello/:name` ที่ทักทายตามชื่อ

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use axum::{Router, routing::get, extract::Path};

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello/:name", get(greet));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

</details>

---

## แบบฝึกหัดที่ 3: JSON Response

Return JSON response

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use axum::{Router, routing::get, Json};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

</details>

---

## แบบฝึกหัดที่ 4: POST with JSON Body

รับ JSON body และ return response

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
    message: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<UserResponse> {
    Json(UserResponse {
        id: 1,
        name: payload.name,
        email: payload.email,
        message: "User created successfully".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

</details>

---

## แบบฝึกหัดที่ 5: Multiple Routes

สร้าง REST API ที่มีหลาย routes

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use axum::{
    Router,
    routing::{get, post, delete},
    extract::Path,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Item {
    id: u32,
    name: String,
}

async fn list_items() -> Json<Vec<Item>> {
    Json(vec![
        Item { id: 1, name: "Item 1".to_string() },
        Item { id: 2, name: "Item 2".to_string() },
    ])
}

async fn get_item(Path(id): Path<u32>) -> Json<Item> {
    Json(Item { id, name: format!("Item {}", id) })
}

#[derive(Deserialize)]
struct CreateItem {
    name: String,
}

async fn create_item(Json(payload): Json<CreateItem>) -> Json<Item> {
    Json(Item { id: 3, name: payload.name })
}

async fn delete_item(Path(id): Path<u32>) -> String {
    format!("Deleted item {}", id)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).delete(delete_item));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("API running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

</details>

---

👉 [บทที่ 20](./ch20-exercises.md)
