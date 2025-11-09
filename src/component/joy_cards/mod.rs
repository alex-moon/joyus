use crate::component::joy_card::JoyCard;
use crate::component::Renderable;
use crate::service::state::AppState;
use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use futures_util::future::join_all;

#[derive(Template)]
#[template(path = "component/joy_cards/joy_cards.html")]
pub struct JoyCards {
    joy_cards: String,
}

#[async_trait::async_trait]
impl Renderable for JoyCards {
    async fn render_with_state(state: &AppState) -> Result<Html<String>, String> {
        let user = state.users.summary().await;
        let mut joys = state.joys.list_for_user(&user.id).await?;
        joys.sort_by(|a, b| b.created.cmp(&a.created));

        let futures = joys.iter().map(|j| {
            let state = state; // capture
            let id = j.id;
            async move {
                JoyCard::render_with_state_id(state, id).await.map(|h| h.0)
            }
        });

        let results: Vec<Result<String, String>> = join_all(futures).await;

        let pieces: Result<Vec<String>, String> = results.into_iter().collect();
        let joined = pieces?.join("");

        let html = JoyCards { joy_cards: joined }
            .render()
            .map_err(|e| e.to_string())?;
        Ok(Html(html))
    }
}

async fn show(state: axum::extract::State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    JoyCards::render_with_state(&state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest(
            "/joy-cards",
            Router::new()
                .route("/", get(show))
        )
}
