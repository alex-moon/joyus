use askama::Template;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::http::StatusCode;
use axum::Router;

use crate::component::questions::Questions;
use crate::service::{
    state::AppState,
};

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {
    questions: String,
}

/// GET /app — compose the app by rendering the Questions page and embedding it
pub async fn show(State(state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let user = state.users.summary().await;

    let questions = Questions { user }
        .render()
        .map_err(crate::service::internal_error)?;
    let app = App { questions };
    let html = app.render().map_err(crate::service::internal_error)?;
    Ok(Html(html))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/app", Router::new().route("/", get(show)))
}
