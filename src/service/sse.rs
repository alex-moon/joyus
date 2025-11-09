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

    pub fn publish_html(&self, html: String) -> Result<usize, broadcast::error::SendError<String>> {
        if self.html_tx.receiver_count() == 0 {
            return Ok(0);
        }
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
            Ok(html) => {
                let cleaned = html.replace(['\n', '\r'], "");
                let data = format!("elements {}", cleaned);
                Some(Ok(Event::default()
                    .event("datastar-patch-elements")
                    .data(data)
                ))
            },
            Err(BroadcastStreamRecvError::Lagged(_)) => None,
        }
    });
    Sse::new(stream)
}
