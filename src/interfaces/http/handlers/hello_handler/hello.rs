use axum::{http::StatusCode, response::IntoResponse, Json as AxumJson};
use serde_json::json;

pub async fn hello() -> impl IntoResponse {
    (
        StatusCode::OK,
        AxumJson(json!({"message": "Hello, World!"})),
    )
}
