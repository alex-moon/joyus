use askama::Template;

#[derive(Template)]
#[template(path = "component/questions/questions.html")]
pub struct Questions {
}

pub fn render() -> String {
    let q = Questions {};
    q.render().unwrap()
}
