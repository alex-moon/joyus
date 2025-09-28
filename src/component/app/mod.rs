pub fn render() -> String {
    // In a more advanced setup, you might compile a template here.
    // For now, we inline the component's HTML at compile time.
    include_str!("app.html").to_string()
}
