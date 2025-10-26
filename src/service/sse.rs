use {
    axum::{extract::State, response::{sse::Event, Sse}},
    core::convert::Infallible,
    futures_util::StreamExt,
    tokio::sync::broadcast,
    tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
};

use crate::service::state::AppState;

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
    State(state): State<AppState>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.sse.subscriber();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(html) => Some(Ok(Event::default().event("component-swap").data(html))),
            Err(BroadcastStreamRecvError::Lagged(_)) => None,
        }
    });
    Sse::new(stream)
}
