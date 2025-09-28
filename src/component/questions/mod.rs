use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "component/questions/questions.html")]
pub struct Questions {
}

pub async fn render() -> String {
    let questions = Questions {};
    questions.render().unwrap()
}

pub async fn component() -> Html<String> {
    Html(render().await)
}
