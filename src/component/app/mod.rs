use askama::Template;
use axum::response::Html;
use crate::component;

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {
    questions: String,
}

pub async fn render() -> String {
    let Html(questions) = component::questions::component().await;
    let app = App {questions};
    app.render().unwrap()
}

pub async fn component() -> Html<String> {
    Html(render().await)
}
