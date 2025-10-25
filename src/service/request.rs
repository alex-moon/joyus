use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{HeaderMap, Request};
use url::form_urlencoded;

use crate::service::state::GlobalState;

/// Per-request context passed to components for rendering.
/// Contains immutable global state and request-scoped data like headers, query, and form data.
pub struct RequestContext {
    pub state: Arc<GlobalState>,
    pub headers: HeaderMap,
    pub query: HashMap<String, String>,
    pub form: HashMap<String, String>,
}

impl RequestContext {
    /// Build a RequestContext from an Axum request without consuming the body.
    /// This fills headers and query; `form` remains empty unless provided separately.
    pub fn from_request_without_body(req: &Request<Body>, state: Arc<GlobalState>) -> Self {
        let headers = req.headers().clone();

        let query_str = req.uri().query().unwrap_or("");
        let mut query = HashMap::new();
        if !query_str.is_empty() {
            for (k, v) in form_urlencoded::parse(query_str.as_bytes()) {
                query.insert(k.into_owned(), v.into_owned());
            }
        }

        Self {
            state,
            headers,
            query,
            form: HashMap::new(),
        }
    }

    /// Optionally attach already-parsed urlencoded form data.
    pub fn with_form(mut self, form: HashMap<String, String>) -> Self {
        self.form = form;
        self
    }
}
