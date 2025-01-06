mod user;

use time::macros::date;
use tokio::sync::Mutex;
use user::{User, UserDTO};

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

use uuid::Uuid;

type AppState = Arc<Mutex<HashMap<Uuid, User>>>;

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

    let shared_state = Arc::new(Mutex::new(users));

    let app = Router::new()
        .route("/", get(hello))
        .route("/users", get(get_users))
        .route("/users/{uuid}", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/users", put(update_user))
        .route("/users", delete(delete_user))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

async fn get_users() -> impl IntoResponse {
    (StatusCode::OK, "Lista com todos os usuários!")
}

async fn get_user_by_id(
    State(users): State<AppState>,
    Path(user_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match users.lock().await.get(&user_uuid) {
        Some(user) => Ok((StatusCode::OK, Json(user.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Usuário com id: {}. Não foi encontrado.", &user_uuid),
        )),
    }
}

async fn create_user(State(users): State<AppState>, Json(new_user): Json<UserDTO>) -> impl IntoResponse {
    let user = User::new(new_user.name, new_user.password, new_user.birth_date, new_user.role);
    
    users.lock().await.insert(user.id, user.clone());

    (StatusCode::CREATED, Json(user))
}

async fn update_user() -> impl IntoResponse {
    (StatusCode::OK, "Updated")
}

async fn delete_user() -> impl IntoResponse {
    (StatusCode::OK, "deleted")
}
