use axum::routing::post;
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
use tower_sessions::{Session, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

mod component;
mod service;

use service::{
    event::EventBus,
    sse::{events as sse_events, SseService},
    state::AppState,
    joy::JoyService,
    user::UserService,
};
use crate::service::user::update_user;

#[derive(Template)]
#[template(path = "../public/index.html")]
struct Index {
    app: String,
}

async fn index(State(state): State<AppState>, session: Session) -> Result<Html<String>, (StatusCode, String)> {
    let Html(app) = component::app::show(State(state), session).await?;
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

    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let session_store = PostgresStore::new(pool.clone());
    match session_store.migrate().await {
        Ok(_) => tracing::info!("tower-sessions migration successful."),
        Err(e) => {
            tracing::error!("tower-sessions migration failed with error: {:?}", e);
            // This will panic and print the full error detail to stdout
            panic!("Fatal Session Migration Error: {:?}", e);
        }
    }
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

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

    let events_router: Router<AppState> = Router::new()
        .route("/events", get(sse_events));

    let user_router: Router<AppState> = Router::new()
        .route("/user", post(update_user));

    let routes = base
        .merge(events_router)
        .merge(user_router)
        .layer(session_layer)
        .with_state(app_state)
        .fallback_service(
            ServeDir::new("public").append_index_html_on_directories(true),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes).await.unwrap();

    Ok(())
}
