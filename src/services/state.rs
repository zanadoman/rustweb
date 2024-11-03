use std::sync::atomic::{AtomicU64, Ordering};

use axum::response::sse::Event;
use sqlx::{Error, MySqlPool};
use tokio::sync::broadcast::{channel, Sender};
use tracing::instrument;

use crate::models::message::MessageModel;

#[derive(Debug)]
pub struct StateService {
    id: AtomicU64,
    database: MySqlPool,
    messages: Sender<(Event, Option<MessageModel>)>,
}

impl StateService {
    #[instrument(level = "debug")]
    pub async fn new(database: &str) -> Result<Self, Error> {
        Ok(Self {
            id: AtomicU64::default(),
            database: MySqlPool::connect(database).await?,
            messages: channel(u8::MAX.into()).0,
        })
    }

    pub fn id(&self) -> u64 {
        self.id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn database(&self) -> &MySqlPool {
        &self.database
    }

    pub fn messages(&self) -> &Sender<(Event, Option<MessageModel>)> {
        &self.messages
    }
}
