use crate::domain::entities::user::User;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub inmemory_state: Arc<RwLock<HashMap<Uuid, User>>>,
    //cache_state: Arc<String>,
}
