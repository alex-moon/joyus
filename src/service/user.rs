use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserSummary {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone)]
pub struct UserService {
    current: Arc<RwLock<UserSummary>>, // pretend authenticated user
}

impl UserService {
    pub fn new() -> Self {
        let user = UserSummary { id: Uuid::new_v4(), name: "Guest".to_string() };
        Self { current: Arc::new(RwLock::new(user)) }
    }

    pub async fn summary(&self) -> UserSummary {
        // trivial async boundary
        self.current.read().await.clone()
    }
}
