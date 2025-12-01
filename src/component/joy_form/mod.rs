use askama::Template;
use axum::extract::{State};
use axum::Json;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use crate::service::state::AppState;
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "component/joy_form/joy_form.html")]
pub struct JoyForm {
    pub user: User,
}

use crate::service::user::User;

pub async fn render_for_session(state: &AppState, session: Session) -> Result<Html<String>, String> {
    let user = state.users.get_or_create_session_user(session).await?;
    let html = JoyForm { user }
        .render()
        .map_err(|e| e.to_string())?;
    Ok(Html(html))
}

pub async fn show(State(state): State<AppState>, session: Session) -> Result<Html<String>, (StatusCode, String)> {
    let user = state.users.get_or_create_session_user(session).await.map_err(crate::service::internal_error)?;

    let tpl = JoyForm { user };
    let html = tpl.render().map_err(crate::service::internal_error)?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct NewJoy {
    frustration: String,
    context: String,
    joy: String,
    longitude: Option<f64>,
    latitude: Option<f64>,
}

pub async fn create(
    State(state): State<AppState>,
    session: Session,
    Json(form): Json<NewJoy>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = state
        .users
        .get_or_create_session_user(session)
        .await
        .map_err(crate::service::internal_error)?;

    if let (Some(lon), Some(lat)) = (form.longitude, form.latitude) {
        // persist the last known location for this user
        if let Err(e) = state.users.update_location(&user.id, lat, lon).await {
            tracing::warn!(error = %e, "failed to update user location");
        }
    }

    let res = state
        .joys
        .create(
            &user.id,
            form.frustration.clone(),
            form.context.clone(),
            form.joy.clone(),
        )
        .await;

    if let Err(err) = res {
        return Err((StatusCode::BAD_REQUEST, err));
    }

    let form = JoyForm { user: user.clone() }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Html(cards) = crate::component::joy_cards::render_for_user(&state, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    if let Err(e) = state.sse.publish_html(cards) {
        tracing::warn!(?e, "failed to publish SSE html");
    }

    Ok(Html(form))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .nest(
            "/joy-form",
            Router::new().route("/", get(show)).route("/", post(create)),
        )
}
