use askama::Template;
use axum::extract::{State};
use axum::Json;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;

use crate::component::Renderable;
use crate::service::state::AppState;
use crate::service::joy::Point;

/// Full Joy Json page template
#[derive(Template)]
#[template(path = "component/joy_form/joy_form.html")]
pub struct JoyForm {
    pub user: UserSummary,
}

use crate::service::user::UserSummary;

#[async_trait::async_trait]
impl Renderable for JoyForm {
    async fn render_with_state(state: &AppState) -> Result<Html<String>, String> {
        let user = state.users.summary().await;
        let html = JoyForm { user }
            .render()
            .map_err(|e| e.to_string())?;
        Ok(Html(html))
    }
}

pub async fn show(State(state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let user = state.users.summary().await;

    let tpl = JoyForm { user };
    let html = tpl.render().map_err(crate::service::internal_error)?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct NewJoy {
    frustration: String,
    context: String,
    joy: String,
    lon: Option<f64>,
    lat: Option<f64>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(form): Json<NewJoy>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = state.users.summary().await;

    let point = match (form.lon, form.lat) {
        (Some(lon), Some(lat)) => Some(Point::new(lon, lat).map_err(|e| (StatusCode::BAD_REQUEST, e))?),
        _ => None,
    };

    let res = state
        .joys
        .create(
            &user.id,
            point,
            form.frustration.clone(),
            form.context.clone(),
            form.joy.clone(),
        )
        .await;

    if let Err(err) = res {
        return Err((StatusCode::BAD_REQUEST, err));
    }

    let html = JoyForm { user }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // if let Err(e) = state.sse.publish_html(html.clone()) {
    //     tracing::warn!(?e, "failed to publish SSE html");
    // }

    Ok(Html(html))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest(
            "/joy-form",
            Router::new().route("/", get(show)).route("/", post(create)),
        )
}
