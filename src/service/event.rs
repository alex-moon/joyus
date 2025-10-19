use tokio::sync::broadcast;

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<String>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _rx) = broadcast::channel(capacity);
        Self { sender }
    }

    #[allow(dead_code)]
    pub fn publish(&self, event: String) -> Result<usize, broadcast::error::SendError<String>> {
        self.sender.send(event)
    }

    #[allow(dead_code)]
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }
}
