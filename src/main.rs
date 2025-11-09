use sqlx::postgres::PgPoolOptions;
use {
    axum::{
        extract::State,
        response::Html,
        routing::{get, get_service},
        http::StatusCode,
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
    state::AppState,
    joy::JoyService,
    user::UserService,
};

#[derive(Template)]
#[template(path = "../public/index.html")]
struct Index {
    app: String,
}

async fn index(State(state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let Html(app) = component::app::show(State(state)).await?;
    let html = Index { app }.render().map_err(service::internal_error)?;
    Ok(Html(html))
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

    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    // Initialize services
    let _event_bus = Arc::new(EventBus::new(100));
    let sse = Arc::new(SseService::new(100));
    let users = Arc::new(UserService::new(pool.clone()));
    let joys = Arc::new(JoyService::new(pool.clone()));

    // App state
    let app_state = AppState {
        users: users.clone(),
        joys: joys.clone(),
        sse: sse.clone(),
    };

    // Build routers (all share the same AppState via with_state)
    let base: Router<AppState> = Router::new()
        .route("/", get(index))
        .merge(component::app::router())
        .merge(component::joy_form::router())
        .merge(component::joy_cards::router())
        .route("/favicon.ico", get_service(ServeFile::new("public/assets/favicon.ico")));

    let events_router: Router<AppState> = Router::new().route("/events", get(sse_events));

    let routes = base
        .merge(events_router)
        .fallback_service(static_service)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes).await.unwrap();

    Ok(())
}
