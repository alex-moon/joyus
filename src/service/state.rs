use std::sync::Arc;

use super::{joy::JoyService, sse::SseService, user::UserService};

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<UserService>,
    pub joys: Arc<JoyService>,
    pub sse: Arc<SseService>,
}
