use askama::Template;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::http::StatusCode;
use axum::Router;

use crate::component::{Renderable, joy_form::JoyForm};
use crate::service::{
    state::AppState,
};

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {
    joy_form: String,
}

/// GET /app — compose the app by rendering the Joy Form page and embedding it
pub async fn show(State(state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let Html(joy_form) = JoyForm::render_with_state(&state)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let app = App { joy_form };
    let html = app.render().map_err(crate::service::internal_error)?;
    Ok(Html(html))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/app", Router::new().route("/", get(show)))
}
