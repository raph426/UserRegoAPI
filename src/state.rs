use crate::models::User;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    // arc for sharing ownership accroos threads
    // rwlock allows for many readers or one writer at a time
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub jwt_secret: Arc<String>,
}

impl AppState {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            jwt_secret: Arc::new(jwt_secret),
        }
    }
}
