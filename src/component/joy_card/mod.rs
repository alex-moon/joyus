use crate::component::Renderable;
use crate::service::joy::Joy;
use crate::service::state::AppState;
use askama::Template;
use axum::response::Html;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "component/joy_card/joy_card.html")]
pub struct JoyCard {
    joy: Joy,
}

impl JoyCard {
    pub async fn render_with_state_id(
        state: &AppState,
        id: Uuid,
    ) -> Result<Html<String>, String> {
        let maybe_joy = state.joys.get(id).await;

        let joy = match maybe_joy {
            Ok(Some(j)) => j,
            Ok(None) => return Err(format!("Joy not found: {}", id)),
            Err(e) => return Err(e),
        };

        let html = JoyCard { joy }
            .render()
            .map_err(|e| e.to_string())?;
        Ok(Html(html))
    }
}

#[async_trait::async_trait]
impl Renderable for JoyCard {
    async fn render_with_state(_state: &AppState) -> Result<Html<String>, String> {
        Err("JoyCard requires an id; use render_with_state_id".to_string())
    }
}