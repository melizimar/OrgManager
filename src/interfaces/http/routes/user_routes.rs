use crate::interfaces::http::handlers::user_handler::{
    create_user, delete_user, get_user_by_id, get_users, import_users_by_csv, update_user,
};
use crate::interfaces::http::state::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users", put(update_user))
        .route("/users/{uuid}", get(get_user_by_id))
        .route("/users/{uuid}", delete(delete_user))
        .route("/users/import", post(import_users_by_csv))
}
