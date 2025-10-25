use askama::Template;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use crate::{component, index};

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {
    questions: String,
}

pub async fn render() -> String {
    let Html(questions) = component::questions::show().await;
    let app = App {questions};
    app.render().unwrap()
}

pub async fn show() -> Html<String> {
    Html(render().await)
}

pub fn router() -> Router {
    Router::new()
        .nest("/app", Router::new()
            .route("/", get(show))
        )
}
