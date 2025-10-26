use std::sync::Arc;

use super::{questions::QuestionsService, sse::SseService, user::UserService};

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<UserService>,
    pub questions: Arc<QuestionsService>,
    pub sse: Arc<SseService>,
}
