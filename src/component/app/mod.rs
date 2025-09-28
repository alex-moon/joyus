use askama::Template;

#[derive(Template)]
#[template(path = "component/app/app.html")]
pub struct App {
}

pub fn render() -> String {
    let app = App {};
    app.render().unwrap()
}
