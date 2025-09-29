use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {}

pub fn render() -> String {
    let app = App {};
    app.render().unwrap()
}

pub async fn component_app() -> Html<String> {
    Html(render())
}
