# Axum พื้นฐาน

**Axum** เป็น web framework จาก Tokio team ออกแบบมาสำหรับ async Rust

## Setup

```toml,ignore
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Hello World

```rust,ignore
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // สร้าง router
    let app = Router::new()
        .route("/", get(hello));

    // สร้าง TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running at http://localhost:3000");

    // รัน server
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}
```

รัน:

```bash
cargo run
```text

เปิดเบราว์เซอร์ไปที่ http://localhost:3000

---

## Routes

### Basic Routes

```rust,ignore
let app = Router::new()
    .route("/", get(index))
    .route("/about", get(about))
    .route("/contact", get(contact));
```

### HTTP Methods

```rust,ignore
use axum::routing::{get, post, put, delete};

let app = Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(get_user).put(update_user).delete(delete_user));
```

### Nested Routes

```rust,ignore
let api_routes = Router::new()
    .route("/users", get(list_users))
    .route("/posts", get(list_posts));

let app = Router::new()
    .nest("/api/v1", api_routes)
    .route("/", get(index));

// ผลลัพธ์:
// GET /api/v1/users
// GET /api/v1/posts
// GET /
```

---

## Handler Functions

### Basic Handler

```rust,ignore
async fn hello() -> &'static str {
    "Hello, World!"
}

async fn json_response() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "message": "Hello JSON"
    }))
}
```

### Return Types

| Type              | Description         |
| ----------------- | ------------------- |
| `&'static str`    | Plain text          |
| `String`          | Dynamic text        |
| `Json\<T\>`       | JSON response       |
| `Html\<String\>`  | HTML response       |
| `(StatusCode, T)` | Custom status       |
| `Result\<T, E\>`  | With error handling |

```rust,ignore
use axum::{response::Html, http::StatusCode};

async fn html_page() -> Html<String> {
    Html("<h1>Hello HTML</h1>".to_string())
}

async fn custom_status() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Created!")
}
```

---

## Extractors

ดึงข้อมูลจาก request:

### Path Parameters

```rust,ignore
use axum::extract::Path;

// GET /users/123
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

// GET /users/123/posts/456
async fn get_user_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("User {} Post {}", user_id, post_id)
}
```

### Query Parameters

```rust,ignore
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    page: Option<u32>,
}

// GET /search?q=rust&page=2
async fn search(Query(params): Query<SearchParams>) -> String {
    format!("Searching: '{}' page {:?}", params.q, params.page)
}
```

### JSON Body

```rust,ignore
use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// POST /users with JSON body
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    Json(user)
}
```

### Headers

```rust,ignore
use axum::http::HeaderMap;

async fn read_headers(headers: HeaderMap) -> String {
    if let Some(auth) = headers.get("authorization") {
        format!("Auth: {:?}", auth)
    } else {
        "No auth header".to_string()
    }
}
```

---

## State

แชร์ข้อมูลระหว่าง handlers:

```rust,ignore
use axum::extract::State;
use std::sync::Arc;

struct AppState {
    db_pool: String,  // ปกติใช้ connection pool จริง
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        db_pool: "postgres://...".to_string(),
    });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);

    // ...
}

async fn handler(State(state): State<Arc<AppState>>) -> String {
    format!("DB: {}", state.db_pool)
}
```

---

## Error Handling

```rust,ignore
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

enum AppError {
    NotFound,
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.leak()),
        };
        (status, message).into_response()
    }
}

async fn fallible_handler() -> Result<String, AppError> {
    // อาจ return Err(AppError::NotFound)
    Ok("Success!".to_string())
}
```

---

## Middleware

```rust,ignore
use axum::middleware;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/", get(handler))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());
```

---

## ลองทำดู! 🎯

1. สร้าง API ที่มี GET และ POST
2. ใช้ Path extractor รับ id
3. ใช้ Json extractor รับ body

---

## 🌍 Complete CRUD API Example

ตัวอย่าง Todo API แบบครบ:

```rust,ignore
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

// Data models
#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

type Db = Arc<RwLock<Vec<Todo>>>;

// Handlers
async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    let todos = db.read().unwrap().clone();
    Json(todos)
}

async fn create_todo(
    State(db): State<Db>,
    Json(input): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = db.write().unwrap();
    let id = todos.len() as u32 + 1;
    let todo = Todo {
        id,
        title: input.title,
        completed: false,
    };
    todos.push(todo.clone());
    (StatusCode::CREATED, Json(todo))
}

async fn get_todo(
    State(db): State<Db>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = db.read().unwrap();
    todos
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_todo(
    State(db): State<Db>,
    Path(id): Path<u32>,
) -> StatusCode {
    let mut todos = db.write().unwrap();
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        todos.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(vec![]));

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).delete(delete_todo))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### 📋 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/todos` | List all todos |
| POST | `/todos` | Create new todo |
| GET | `/todos/:id` | Get todo by ID |
| DELETE | `/todos/:id` | Delete todo |

### 🚀 Deployment Checklist

```text
┌─────────────────────────────────────────────────────────────────────┐
│                    Production Deployment                             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│   1. Build Release                                                   │
│      cargo build --release                                           │
│                                                                       │
│   2. Dockerfile                                                      │
│      FROM rust:1.75 as builder                                       │
│      WORKDIR /app                                                    │
│      COPY . .                                                        │
│      RUN cargo build --release                                       │
│                                                                       │
│      FROM debian:bookworm-slim                                       │
│      COPY --from=builder /app/target/release/myapp /usr/local/bin   │
│      CMD ["myapp"]                                                   │
│                                                                       │
│   3. Environment Variables                                           │
│      DATABASE_URL=postgres://...                                     │
│      PORT=3000                                                       │
│                                                                       │
│   4. Deploy Options                                                  │
│      • Docker → AWS ECS / Google Cloud Run                          │
│      • Binary → VPS (DigitalOcean, Linode)                          │
│      • Serverless → AWS Lambda + cargo-lambda                       │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## สรุป

| Concept | Example                         |
| ------- | ------------------------------- |
| Route   | `.route("/path", get(handler))` |
| Path    | `Path(id): Path<u32>`           |
| Query   | `Query(params): Query<T>`       |
| JSON    | `Json(body): Json<T>`           |
| State   | `State(state): State<Arc<T>>`   |
| Nested  | `.nest("/api", routes)`         |

👉 ต่อไป: [สร้าง REST API](./03-api-example.md)
