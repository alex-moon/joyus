use {
    axum::{
        response::Html,
        routing::{get, get_service},
        Router,
    },
    askama::Template,
    core::error::Error,
    std::sync::Arc,
    tower_http::services::{ServeDir, ServeFile},
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

mod component;
mod service;

use service::{
    event::EventBus,
    sse::{events as sse_events, SseService},
};


#[derive(Template)]
#[template(path = "../public/index.html")]
struct Index {
    app: String,
}

async fn index() -> Html<String> {
    let Html(app) = component::app::component().await;
    Html(Index { app }.render().unwrap())
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

    // Initialize services
    let _event_bus = Arc::new(EventBus::new(100));
    let sse = Arc::new(SseService::new(100));

    // Build routers
    let base = Router::new()
        .route("/", get(index))
        .route("/app", get(component::app::component))
        .route("/favicon.ico", get_service(ServeFile::new("public/assets/favicon.ico")));

    let events_router = Router::new()
        .route("/events", get(sse_events))
        .with_state(sse);

    let app = base
        .merge(events_router)
        .fallback_service(static_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
