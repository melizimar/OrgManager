use crate::domain::entities::user::User;
use crate::interfaces::http::state::AppState;
use crate::interfaces::dtos::user_dto::UserDTO;

use std::path;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use serde_json::json;

use uuid::Uuid;

use axum::{
    body::Body,
    extract::{Json, Multipart, Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::*,
    Router
};

#[allow(unused)]
use std::{collections::HashMap, sync::Arc};

#[allow(unused)]
use tokio::sync::RwLock;

#[allow(unused)]
use time::macros::date;

#[allow(unused)]
use tower::ServiceExt;

#[allow(unused)]
use hyper::Response;

pub async fn get_users(State(users): State<AppState>) -> impl IntoResponse {
    match users.inmemory_state.try_read() {
        Ok(state) => {
            let user_list: Vec<User> = state.values().cloned().collect();
            Ok((StatusCode::OK, Json(user_list)))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                json!({"error": format!("Could not access state, INTERNAL_SERVER_ERROR: {}", err)}),
            ),
        )),
    }
}

pub async fn get_user_by_id(
    State(users): State<AppState>,
    Path(user_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match users.inmemory_state.read().await.get(&user_uuid) {
        Some(user) => Ok((StatusCode::OK, Json(user.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"message": "User not found."})),
        )),
    }
}

pub async fn create_user(
    State(users): State<AppState>,
    Json(new_user): Json<UserDTO>,
) -> impl IntoResponse {
    let user = User::new(
        new_user.name,
        new_user.password,
        new_user.birth_date,
        new_user.role,
    );

    users
        .inmemory_state
        .write()
        .await
        .insert(user.id, user.clone());

    (StatusCode::CREATED, Json(user))
}

pub async fn update_user() -> impl IntoResponse {
    (StatusCode::OK, "Updated")
}

pub async fn import_users_by_csv(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)
        .unwrap()
    {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let file_extension = match path::Path::new(&file_name).extension() {
            Some(ext) => format!("{}", ext.to_string_lossy()),
            None => String::from(""),
        };

        println!(
            "----------------\nname: {}\nfile_name: {}\nfile_extension: {}",
            name, file_name, file_extension
        );

        if &name == "csv" && &file_extension == "csv" {
            let output = format!(
                "./src/public/uploads/import/users/{}-{}",
                uuid::Uuid::now_v7(),
                file_name.clone()
            );

            let path = path::Path::new(&output);

            // Verifica se o caminho do arquivo é válido e possui um diretório pai
            if let Some(parent) = path.parent() {
                // Garante que os diretórios pai existem
                fs::create_dir_all(parent).await.unwrap();
            }

            let data = field.bytes().await.unwrap();
            let mut file = fs::File::create(path).await.unwrap();
            file.write(&data).await.unwrap();
        }
    }

    StatusCode::ACCEPTED
}

pub async fn delete_user(
    State(users): State<AppState>,
    Path(user_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match users.inmemory_state.write().await.remove(&user_uuid) {
        Some(_user) => Ok(StatusCode::NO_CONTENT),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"message": "User not found."})),
        )),
    }
}

#[tokio::test]
async fn test_get_users_route() {
    let mut users: HashMap<Uuid, User> = HashMap::new();

    let user1 = User::new("User1", "Password1", date!(1990 - 01 - 01), "Admin");
    let user2 = User::new("User2", "Password2", date!(1995 - 05 - 05), "Standard");

    users.insert(user1.id, user1.clone());
    users.insert(user2.id, user2.clone());

    let shared_state = AppState {
        inmemory_state: Arc::new(RwLock::new(users)),
    };

    // Criar o router
    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(shared_state);

    // Simular requisição GET para a rota /users
    let request = Request::builder()
        .uri("/users")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response: Response<Body> = app.clone().oneshot(request).await.unwrap();

    // Verificar o status HTTP
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_user_route() {
    let mut users: HashMap<Uuid, User> = HashMap::new();

    let user1 = User::new("User1", "Password1", date!(1990 - 01 - 01), "Admin");
    let user2 = User::new("User2", "Password2", date!(1995 - 05 - 05), "Standard");

    users.insert(user1.id, user1.clone());
    users.insert(user2.id, user2.clone());

    let shared_state = AppState {
        inmemory_state: Arc::new(RwLock::new(users)),
    };

    // Criar o router
    let app = Router::new()
        .route("/users/{uuid}", delete(delete_user))
        .with_state(shared_state);

    let uri = format!("/users/{}", user1.id);
    // Simular requisição GET para a rota /users
    let request = Request::builder()
        .uri(&uri)
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response: Response<Body> = app.clone().oneshot(request).await.unwrap();

    println!("TESTE");
    // Verificar o status HTTP
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
