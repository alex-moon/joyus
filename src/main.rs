use {
    asynk_strim::{stream_fn, Yielder},
    axum::{
        response::{sse::Event, Html, IntoResponse, Sse},
        routing::{get, get_service},
        Router,
    },
    askama::Template,
    core::{convert::Infallible, error::Error, time::Duration},
    datastar::{axum::ReadSignals, prelude::PatchElements},
    serde::Deserialize,
    tower_http::services::ServeDir,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

mod component;

#[derive(Template)]
#[template(path = "../public/index.html")]
struct Index {
    app_content: String,
}

async fn index() -> Html<String> {
    // Use the app handler to render the app component so any future handler logic is executed
    let Html(app_content) = component_app().await;
    Html(Index { app_content }.render().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let static_service = get_service(ServeDir::new("public").append_index_html_on_directories(true));

    let app = Router::new()
        .route("/", get(index))
        .route("/app", get(component_app))
        .route("/events", get(events))
        .fallback_service(static_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn component_app() -> Html<String> {
    Html(component::app::render())
}

const MESSAGE: &str = "Hello, world!";

#[derive(Deserialize)]
pub struct Signals {
    pub delay: u64,
}

async fn events(ReadSignals(signals): ReadSignals<Signals>) -> impl IntoResponse {
    Sse::new(stream_fn(
        move |mut yielder: Yielder<Result<Event, Infallible>>| async move {
            for i in 0..MESSAGE.len() {
                let elements = format!("<div id='message'>{}</div>", &MESSAGE[0..i + 1]);
                let patch = PatchElements::new(elements);
                let sse_event = patch.write_as_axum_sse_event();

                yielder.yield_item(Ok(sse_event)).await;

                tokio::time::sleep(Duration::from_millis(signals.delay)).await;
            }
        },
    ))
}
