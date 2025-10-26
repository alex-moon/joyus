pub mod event;
pub mod sse;
pub mod state;
pub mod user;
pub mod joy;

use axum::http::StatusCode;

pub fn internal_error<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
