use axum::response::Html;

use crate::service::state::AppState;

pub mod app;
pub mod joy_form;

#[async_trait::async_trait]
pub trait Renderable {
    async fn render_with_state(state: &AppState) -> Result<Html<String>, String>;
}
