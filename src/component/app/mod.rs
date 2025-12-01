use askama::Template;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::http::StatusCode;
use axum::Router;
use tower_sessions::Session;

use crate::service::{
    state::AppState,
};

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {
    joy_form: String,
    joy_cards: String,
}

pub async fn show(State(state): State<AppState>, session: Session) -> Result<Html<String>, (StatusCode, String)> {
    let user = state
        .users
        .get_or_create_session_user(session.clone())
        .await
        .map_err(crate::service::internal_error)?;

    let Html(joy_form) = crate::component::joy_form::render_for_session(&state, session)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let Html(joy_cards) = crate::component::joy_cards::render_for_user(&state, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let app = App { joy_form, joy_cards };
    let html = app.render().map_err(crate::service::internal_error)?;
    Ok(Html(html))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/app", Router::new().route("/", get(show)))
}
