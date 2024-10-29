use axum::response::sse::Event;
use sqlx::{Error, MySqlPool};
use tokio::sync::broadcast::{channel, Sender};
use tracing::instrument;

#[derive(Debug)]
pub struct StateService {
    pub database: MySqlPool,
    pub messages: Sender<Event>,
}

impl StateService {
    #[instrument(level = "debug")]
    pub async fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            database: MySqlPool::connect(url).await?,
            messages: channel(u8::MAX.into()).0,
        })
    }
}
