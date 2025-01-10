use crate::interfaces::http::handlers::hello_handler::hello::hello;
use crate::interfaces::http::state::AppState;

use axum::{routing::any, Router};

pub fn hello_routes() -> Router<AppState> {
    Router::new()
        .route("/hello", any(hello))
}