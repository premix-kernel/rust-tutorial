# เชื่อมต่อ Database

ใช้ SQLx เชื่อมต่อ PostgreSQL, MySQL, หรือ SQLite

## Setup

```toml
# Cargo.toml
[dependencies]
# PostgreSQL
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# หรือ SQLite (ง่ายกว่าสำหรับพัฒนา)
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }

# หรือ MySQL
sqlx = { version = "0.7", features = ["runtime-tokio", "mysql"] }

tokio = { version = "1", features = ["full"] }
```

---

## SQLite Example (Beginner-friendly)

```rust
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create database file if not exists
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:todos.db?mode=create")
        .await?;

    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(&pool)
    .await?;

    println!("Database ready!");

    Ok(())
}
```

---

## PostgreSQL Connection Pool

```rust
use sqlx::postgres::{PgPool, PgPoolOptions};

async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@localhost/mydb".to_string());

    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await.expect("Failed to create pool");
    println!("Connected to database!");
}
```

---

## Basic Queries

### Execute (INSERT, UPDATE, DELETE)

```rust
use sqlx::PgPool;

async fn insert_todo(pool: &PgPool, title: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO todos (title, completed) VALUES ($1, false)"
    )
    .bind(title)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

async fn update_todo(pool: &PgPool, id: i32, completed: bool) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE todos SET completed = $1 WHERE id = $2"
    )
    .bind(completed)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

async fn delete_todo(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
```

### Fetch with FromRow

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

async fn get_all_todos(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
        .fetch_all(pool)
        .await
}

async fn get_todo_by_id(pool: &PgPool, id: i32) -> Result<Option<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

async fn get_completed_todos(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE completed = true")
        .fetch_all(pool)
        .await
}
```

---

## query_as! Macro (Compile-time checked)

```rust
// ต้อง set DATABASE_URL env var
// และรัน: cargo sqlx prepare

async fn get_users_checked(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, name, email FROM users WHERE active = true"#
    )
    .fetch_all(pool)
    .await
}

async fn create_user_checked(
    pool: &PgPool,
    name: &str,
    email: &str
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email
        "#,
        name,
        email
    )
    .fetch_one(pool)
    .await
}
```

---

## Transactions

```rust
use sqlx::PgPool;

async fn transfer_money(
    pool: &PgPool,
    from_id: i32,
    to_id: i32,
    amount: f64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Deduct from sender
    sqlx::query("UPDATE accounts SET balance = balance - $1 WHERE id = $2")
        .bind(amount)
        .bind(from_id)
        .execute(&mut *tx)
        .await?;

    // Add to receiver
    sqlx::query("UPDATE accounts SET balance = balance + $1 WHERE id = $2")
        .bind(amount)
        .bind(to_id)
        .execute(&mut *tx)
        .await?;

    // Commit transaction
    tx.commit().await?;

    Ok(())
}
```

---

## Integration with Axum

```rust
use axum::{
    extract::{State, Path, Json},
    routing::get,
    Router,
    http::StatusCode,
};
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(todos)
}

async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING *"
    )
    .bind(&input.title)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgres://user:pass@localhost/mydb")
        .await
        .expect("Failed to connect");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

---

## Migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Create migration
sqlx migrate add create_todos_table

# Edit migration file: migrations/20231225_create_todos_table.sql
# CREATE TABLE todos (...)

# Run migrations
sqlx migrate run
```

---

## ลองทำดู! 🎯

1. สร้าง SQLite database และ table
2. เขียน CRUD functions
3. เชื่อมต่อกับ Axum handlers

---

## สรุปบทที่ 19

| Concept       | Description               |
| ------------- | ------------------------- |
| `PgPool`      | Connection pool           |
| `query()`     | Basic query               |
| `query_as()`  | Query with struct mapping |
| `query_as!()` | Compile-time checked      |
| `FromRow`     | Derive for struct         |
| `begin()`     | Start transaction         |
| `commit()`    | Commit transaction        |

### Error Handling

```rust
match result {
    Ok(todo) => println!("Got: {:?}", todo),
    Err(sqlx::Error::RowNotFound) => println!("Not found"),
    Err(e) => eprintln!("Database error: {}", e),
}
```

👉 ต่อไป: [บทที่ 20: Final Project](../ch20-final-project/README.md)
