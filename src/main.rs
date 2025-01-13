mod domain;
mod interfaces;

use domain::entities::user::User;

use interfaces::http::routes::hello_routes::*;
use interfaces::http::routes::user_routes::*;
use interfaces::http::state::AppState;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use uuid::Uuid;

use axum::Router;

#[tokio::main]
async fn main() {
    let mut users: HashMap<Uuid, User> = HashMap::new();

    let user1 = User::new("User1", "Password1", "Admin");
    let user2 = User::new("User2", "Password2", "Standard");

    users.insert(user1.id, user1.clone());
    users.insert(user2.id, user2.clone());

    let shared_state: AppState = AppState {
        inmemory_state: Arc::new(RwLock::new(users)),
    };

    let app = Router::new()
        .merge(hello_routes())
        .merge(user_routes())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
