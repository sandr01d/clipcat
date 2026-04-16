use clipcat_proto as proto;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{Response, Status};

use crate::history::HistoryManager;

pub struct HistoryService {
    history: Arc<Mutex<HistoryManager>>,
}

impl HistoryService {
    pub const fn new(history: Arc<Mutex<HistoryManager>>) -> Self {
        Self { history }
    }
}

#[tonic::async_trait]
impl proto::History for HistoryService {
    // TODO: can we get rid of the request parameter?
    async fn clear(&self, _request: tonic::Request<()>) -> Result<Response<()>, Status> {
        let mut history = self.history.lock().await;
        // TODO: proper error handling
        if let Err(err) = history.clear().await {
            tracing::error!("Failed to clear history: {err}");
        }
        Ok(Response::new(()))
    }
}
