# สร้าง REST API

สร้าง CRUD API ด้วย Axum

## Project Setup

```toml
# Cargo.toml
[package]
name = "todo-api"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1", features = ["v4", "serde"] }
```

---

## Data Model

```rust,ignore
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
```

---

## Application State

```rust,ignore
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

fn create_db() -> Db {
    Arc::new(RwLock::new(HashMap::new()))
}
```

ใช้ `RwLock` แทน `Mutex` เพราะ:

- อ่านได้หลาย concurrent readers
- เขียนได้ทีละคน

---

## Complete API

```rust,ignore
use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

// === Models ===

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

// === Handlers ===

// GET /todos
async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    let todos = db.read().await;
    let list: Vec<Todo> = todos.values().cloned().collect();
    Json(list)
}

// GET /todos/:id
async fn get_todo(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = db.read().await;

    todos
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /todos
async fn create_todo(
    State(db): State<Db>,
    Json(input): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let todo = Todo {
        id: Uuid::new_v4(),
        title: input.title,
        description: input.description,
        completed: false,
    };

    db.write().await.insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

// PUT /todos/:id
async fn update_todo(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = db.write().await;

    let todo = todos.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = input.title {
        todo.title = title;
    }
    if let Some(description) = input.description {
        todo.description = Some(description);
    }
    if let Some(completed) = input.completed {
        todo.completed = completed;
    }

    Ok(Json(todo.clone()))
}

// DELETE /todos/:id
async fn delete_todo(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> StatusCode {
    let mut todos = db.write().await;

    if todos.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// === Main ===

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
```

---

## Testing with curl

```bash
# Create todo
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "description": "Complete the tutorial"}'

# List all todos
curl http://localhost:3000/todos

# Get single todo
curl http://localhost:3000/todos/{id}

# Update todo
curl -X PUT http://localhost:3000/todos/{id} \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# Delete todo
curl -X DELETE http://localhost:3000/todos/{id}
```

---

## Error Handling

```rust,ignore
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
enum AppError {
    NotFound,
    InvalidInput(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.leak()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.leak()),
        };

        (status, Json(serde_json::json!({
            "error": message
        }))).into_response()
    }
}

// Use in handler
async fn get_todo_v2(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> Result<Json<Todo>, AppError> {
    let todos = db.read().await;

    todos
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(AppError::NotFound)
}
```

---

## Add Middleware (CORS, Logging)

```rust,ignore
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/todos", get(list_todos).post(create_todo))
    .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
    .with_state(db)
    .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
    .layer(TraceLayer::new_for_http());
```text

---

## ลองทำดู! 🎯

1. เพิ่ม field `priority` ใน Todo
2. เพิ่ม endpoint `GET /todos?completed=true` filter
3. เพิ่ม validation สำหรับ title (ไม่ว่าง)

---

## สรุป

| Endpoint     | Method | Description |
| ------------ | ------ | ----------- |
| `/todos`     | GET    | List all    |
| `/todos`     | POST   | Create      |
| `/todos/:id` | GET    | Get one     |
| `/todos/:id` | PUT    | Update      |
| `/todos/:id` | DELETE | Delete      |

👉 ต่อไป: [เชื่อมต่อ Database](./04-database.md)
