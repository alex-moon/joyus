use crate::component::joy_card::JoyCard;
use crate::service::state::AppState;
use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use futures_util::future::join_all;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "component/joy_cards/joy_cards.html")]
pub struct JoyCards {
    joy_cards: String,
}

pub async fn render_for_user(state: &AppState, user_id: Uuid) -> Result<Html<String>, String> {
    let joys = state.joys.list_for_user(&user_id).await?;

    let futures = joys.iter().map(|j| {
        let state = state; // capture
        let id = j.id;
        async move {
            JoyCard::render_with_state_id(state, id, user_id).await.map(|h| h.0)
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

async fn show(state: axum::extract::State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    // This route should not be used without session-bound user, keep old behavior disabled
    Err((StatusCode::UNAUTHORIZED, "session required".into()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest(
            "/joy-cards",
            Router::new()
                .route("/", get(show))
        )
}
