use axum::{Router, routing::{post, get, put, delete}};
use crate::interfaces::http::handlers::user_handler::{get_users, get_user_by_id, create_user, update_user, delete_user, import_users_by_csv};
use crate::interfaces::http::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users", put(update_user))
        .route("/users/{uuid}", get(get_user_by_id))
        .route("/users/{uuid}", delete(delete_user))
        .route("/users/import", post(import_users_by_csv))
}
