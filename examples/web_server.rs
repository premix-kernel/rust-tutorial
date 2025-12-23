// ===========================================
// Web Server - บทที่ 19
// รัน: cargo run --example web_server
// เปิด: http://localhost:3000
// ===========================================

use axum::{
    Router,
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Shared state
struct AppState {
    users: Mutex<Vec<User>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct QueryParams {
    page: Option<u32>,
    limit: Option<u32>,
}

#[tokio::main]
async fn main() {
    println!("🦀 Web Server Demo\n");

    // Initialize state with sample data
    let state = Arc::new(AppState {
        users: Mutex::new(vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        ]),
    });

    // Build router
    let app = Router::new()
        // Basic routes
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/hello/:name", get(hello_name))
        // User routes
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        // Query params example
        .route("/search", get(search))
        // State
        .with_state(state);

    println!("🚀 Server running on http://localhost:3000");
    println!("");
    println!("Try these endpoints:");
    println!("  GET  /              - Welcome message");
    println!("  GET  /hello         - Hello World");
    println!("  GET  /hello/:name   - Personalized greeting");
    println!("  GET  /users         - List all users");
    println!("  POST /users         - Create user (JSON body)");
    println!("  GET  /users/:id     - Get user by ID");
    println!("  DELETE /users/:id   - Delete user");
    println!("  GET  /search?page=1&limit=10 - Query params");
    println!("");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// Handlers

async fn root() -> &'static str {
    "🦀 Welcome to Rust Web Server!"
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn list_users(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    Json(users.clone())
}

async fn get_user(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>, (StatusCode, String)> {
    let users = state.users.lock().unwrap();

    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, format!("User {} not found", id)))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUser>,
) -> impl IntoResponse {
    let mut users = state.users.lock().unwrap();

    let id = users.len() as u32 + 1;
    let user = User {
        id,
        name: input.name,
        email: input.email,
    };

    users.push(user.clone());

    (StatusCode::CREATED, Json(user))
}

async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut users = state.users.lock().unwrap();
    let len = users.len();
    users.retain(|u| u.id != id);

    if users.len() < len {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, format!("User {} not found", id)))
    }
}

async fn search(Query(params): Query<QueryParams>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("Searching page {} with limit {}", page, limit)
}
