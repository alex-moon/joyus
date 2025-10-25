use askama::Template;
use axum::response::Html;
use axum::Router;
use axum::routing::{get, post};

#[derive(Template)]
#[template(path = "component/questions/questions.html")]
pub struct Questions {
}

pub async fn render() -> String {
    let questions = Questions {};
    questions.render().unwrap()
}

pub async fn show() -> Html<String> {
    Html(render().await)
}

pub async fn create() -> Html<String> {
    Html(render().await)
}

pub fn router() -> Router {
    Router::new()
        .nest("/questions", Router::new()
            .route("/", get(show))
            .route("/", post(create))
        )
}
