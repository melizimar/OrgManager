mod user;

use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;
use serde_json::json;
use time::macros::date;
use tokio::sync::RwLock;
use user::{User, UserDTO};

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    extract::{Json, Path, State},
    routing::{delete, get, post, put},
};

type AppState = Arc<RwLock<HashMap<Uuid, User>>>;

#[tokio::main]
async fn main() {
    let mut users: HashMap<Uuid, User> = HashMap::new();

    let user = User::new(
        "Matheus",
        "Senha123",
        date!(1999 - 10 - 12),
        "Administrator",
    );

    println!("http://localhost:3000/users/{}", &user.id);

    users.insert(user.id, user);

    let user1 = User::new("Teste", "Senha123", date!(1999 - 10 - 15), "Padrão");

    println!("http://localhost:3000/users/{}", &user1.id);

    users.insert(user1.id, user1);

    let shared_state = Arc::new(RwLock::new(users));

    let app = Router::new()
        .route("/", get(hello))
        .route("/users", get(get_users))
        .route("/users/{uuid}", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/users", put(update_user))
        .route("/users/{uuid}", delete(delete_user))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

async fn get_users(State(users): State<AppState>) -> impl IntoResponse {
    let user_list: Vec<User> = users.read().await.values().cloned().collect();
    (StatusCode::OK, Json(user_list))
}

async fn get_user_by_id(
    State(users): State<AppState>,
    Path(user_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match users.read().await.get(&user_uuid) {
        Some(user) => Ok((StatusCode::OK, Json(user.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"message": "User not found."})),
        )),
    }
}

async fn create_user(
    State(users): State<AppState>,
    Json(new_user): Json<UserDTO>,
) -> impl IntoResponse {
    let user = User::new(
        new_user.name,
        new_user.password,
        new_user.birth_date,
        new_user.role,
    );

    users.write().await.insert(user.id, user.clone());

    (StatusCode::CREATED, Json(user))
}

async fn update_user() -> impl IntoResponse {
    (StatusCode::OK, "Updated")
}

async fn delete_user(
    State(users): State<AppState>,
    Path(user_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match users.write().await.remove(&user_uuid) {
        Some(_user) => Ok(StatusCode::NO_CONTENT),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"message": "User not found."})),
        )),
    }
}
