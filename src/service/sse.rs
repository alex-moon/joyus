use {
    asynk_strim::{stream_fn, Yielder},
    axum::{extract::State, response::{sse::Event, IntoResponse, Sse}},
    core::convert::Infallible,
    std::sync::Arc,
    tokio::sync::broadcast,
};

#[derive(Clone)]
pub struct SseService {
    html_tx: broadcast::Sender<String>,
}

impl SseService {
    pub fn new(capacity: usize) -> Self {
        let (html_tx, _rx) = broadcast::channel(capacity);
        Self { html_tx }
    }

    #[allow(dead_code)]
    pub fn publish_html(&self, html: String) -> Result<usize, broadcast::error::SendError<String>> {
        self.html_tx.send(html)
    }

    pub fn subscriber(&self) -> broadcast::Receiver<String> {
        self.html_tx.subscribe()
    }
}

pub async fn events(
    State(sse): State<Arc<SseService>>,
) -> impl IntoResponse {
    let mut rx = sse.subscriber();

    Sse::new(stream_fn(move |mut yielder: Yielder<Result<Event, Infallible>>| async move {
        loop {
            match rx.recv().await {
                Ok(html) => {
                    // Send a plain HTML component swap event via Axum SSE
                    let sse_event = Event::default()
                        .event("component-swap")
                        .data(html);
                    yielder.yield_item(Ok(sse_event)).await;
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    // On lag, just continue and try to receive the next message
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    }))
}
