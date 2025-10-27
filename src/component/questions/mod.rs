use askama::Template;
use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;

use crate::component::Renderable;
use crate::service::{
    user::UserSummary,
    state::AppState,
};

/// Full Questions page template
#[derive(Template)]
#[template(path = "component/questions/questions.html")]
pub struct Questions {
    pub user: UserSummary,
}

#[async_trait::async_trait]
impl Renderable for Questions {
    async fn render_with_state(state: &AppState) -> Result<Html<String>, String> {
        let user = state.users.summary().await;
        let html = Questions { user }
            .render()
            .map_err(|e| e.to_string())?;
        Ok(Html(html))
    }
}

/// GET /questions — gather data and render the full questions section
pub async fn show(State(state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let user = state.users.summary().await;

    let tpl = Questions { user };
    let html = tpl.render().map_err(crate::service::internal_error)?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct NewQuestion {
    title: String,
}

/// POST /questions — create a new question, publish SSE with list fragment, and return latest HTML
pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<NewQuestion>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = state.users.summary().await;

    let title = form.title.trim();
    if title.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "title cannot be empty".into()));
    }

    state.questions.create(&user.id, title.to_string()).await;

    let items = state.questions.list_for_user(&user.id).await;
    let html = Questions { user }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Err(e) = state.sse.publish_html(html.clone()) {
        tracing::warn!(?e, "failed to publish SSE html");
    }

    Ok(Html(html))
}

pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/questions",
        Router::new().route("/", get(show).post(create)),
    )
}
