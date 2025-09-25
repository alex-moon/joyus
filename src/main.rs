use std::sync::{Arc, Mutex};

use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, sse::{Event, KeepAlive, Sse}},
    routing::{get, post},
    Router,
};
use futures_util::stream::Stream;
use serde::Deserialize;
use tokio::sync::broadcast;

#[derive(Clone)]
struct AppState {
    last_text: Arc<Mutex<String>>,            // stores the last submitted text
    tx: broadcast::Sender<String>,            // SSE broadcast channel
}

#[tokio::main]
async fn main() {
    // broadcast channel for SSE
    let (tx, _rx) = broadcast::channel::<String>(16);

    let state = AppState {
        last_text: Arc::new(Mutex::new(String::new())),
        tx,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/submit", post(submit))
        .route("/events", get(events))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let current = {
        // Clone current value for initial render
        let guard = state.last_text.lock().unwrap();
        guard.clone()
    };

    Html(render_page(&current))
}

#[derive(Deserialize, Debug)]
struct SubmitForm {
    text: String,
}

async fn submit(State(state): State<AppState>, Form(form): Form<SubmitForm>) -> impl IntoResponse {
    {
        let mut guard = state.last_text.lock().unwrap();
        *guard = form.text.clone();
    }

    // Broadcast to any connected SSE clients
    let _ = state.tx.send(form.text);

    // Respond with a simple 204 No Content for htmx-style submissions or redirect for normal forms
    StatusCode::NO_CONTENT
}

async fn events(State(state): State<AppState>) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    // Each subscriber gets its own receiver
    let mut rx = state.tx.subscribe();

    let stream = async_stream::stream! {
        // On new connection, immediately send current state so the client renders right away
        let initial = {
            let guard = state.last_text.lock().unwrap();
            guard.clone()
        };
        yield Ok(Event::default().data(initial));

        loop {
            match rx.recv().await {
                Ok(msg) => {
                    yield Ok(Event::default().data(msg));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    // If lagged, continue to next
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

fn render_page(current: &str) -> String {
    let current_html = if current.trim().is_empty() {
        "<em>No text submitted yet.</em>".to_string()
    } else {
        html_escape::encode_safe(current).to_string()
    };

    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Joyus - SSE Demo</title>
    <style>
      body {{ font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, "Apple Color Emoji", "Segoe UI Emoji"; margin: 2rem; background: #0b1021; color: #e1e7ef; }}
      .container {{ display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; align-items: start; }}
      form textarea {{ width: 100%; min-height: 10rem; padding: 0.75rem; border-radius: 0.5rem; border: 1px solid #2c3557; background: #0f1631; color: #e1e7ef; }}
      form button {{ margin-top: 0.75rem; padding: 0.5rem 0.9rem; background: #3b82f6; color: white; border: 0; border-radius: 0.5rem; cursor: pointer; }}
      .card {{ background: #0f1631; border: 1px solid #2c3557; border-radius: 0.75rem; padding: 1rem; }}
      .muted {{ color: #a7b3c6; }}
    </style>
  </head>
  <body>
    <h1>Joyus: SSE Text Swap</h1>
    <div class="container">
      <section>
        <h2 class="muted">1. Submit Text</h2>
        <form action="/submit" method="post" id="text-form">
          <textarea name="text" placeholder="Type something and press Submit..."></textarea>
          <div>
            <button type="submit">Submit</button>
          </div>
        </form>
      </section>
      <section>
        <h2 class="muted">2. Last Submitted</h2>
        <div class="card" id="last-card">{current_html}</div>
      </section>
    </div>

    <script>
      // Submit via fetch to avoid page reload and keep it simple
      const form = document.getElementById('text-form');
      form.addEventListener('submit', async (e) => {{
        e.preventDefault();
        const fd = new FormData(form);
        await fetch('/submit', {{ method: 'POST', body: new URLSearchParams(fd) }});
      }});

      // Connect to SSE and swap card content on each message
      const card = document.getElementById('last-card');
      const ev = new EventSource('/events');
      ev.onmessage = (evt) => {{
        const text = evt.data || '';
        card.textContent = text.trim() ? text : 'No text submitted yet.';
      }};
    </script>
  </body>
</html>"#
    )
}
