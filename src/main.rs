use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/users", put(update_user))
        .route("/users", delete(delete_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

async fn get_users() -> impl IntoResponse {
    (StatusCode::OK, "Lista com todos os usuários!")
}

async fn get_user_by_id() -> impl IntoResponse {
    (StatusCode::OK, "User: id")
}

async fn create_user() -> impl IntoResponse {
    (StatusCode::CREATED, "Created")
}

async fn update_user() -> impl IntoResponse {
    (StatusCode::OK, "Updated")
}

async fn delete_user() -> impl IntoResponse {
    (StatusCode::OK, "deleted")
}
