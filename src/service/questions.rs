use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Question {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
}

#[derive(Clone, Default)]
pub struct QuestionsService {
    // very simple in-memory store keyed by user id
    inner: Arc<RwLock<HashMap<Uuid, Vec<Question>>>>,
}

impl QuestionsService {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn list_for_user(&self, user_id: &Uuid) -> Vec<Question> {
        let map = self.inner.read().await;
        map.get(user_id)
            .cloned()
            .unwrap_or_default()
    }

    pub async fn create(&self, user_id: &Uuid, title: String) {
        let mut map = self.inner.write().await;
        let entry = map.entry(*user_id).or_default();
        entry.push(Question { id: Uuid::new_v4(), user_id: *user_id, title });
    }
}
