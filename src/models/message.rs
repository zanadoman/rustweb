use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query, query_as, Error, MySqlPool};
use tracing::instrument;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MessageModel {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

impl MessageModel {
    #[instrument(skip(database))]
    pub async fn find(
        database: &MySqlPool,
        id: i32,
    ) -> Result<Option<Self>, Error> {
        query_as!(
            MessageModel,
            "SELECT * FROM messages WHERE id = ? LIMIT 1",
            id
        )
        .fetch_optional(database)
        .await
    }

    #[instrument(skip(database))]
    pub async fn all(database: &MySqlPool) -> Result<Vec<Self>, Error> {
        query_as!(MessageModel, "SELECT * FROM messages")
            .fetch_all(database)
            .await
    }

    #[instrument(skip(database))]
    pub async fn create(
        database: &MySqlPool,
        title: &String,
        content: &String,
    ) -> Result<u64, Error> {
        query!(
            "INSERT INTO messages (title, content) VALUES (?, ?)",
            title,
            content
        )
        .execute(database)
        .await
        .map(|row| row.last_insert_id())
    }

    #[instrument(skip(database))]
    pub async fn update(
        &self,
        database: &MySqlPool,
        title: Option<&String>,
        content: Option<&String>,
    ) -> Result<(), Error> {
        query!(
            "UPDATE messages SET title = ?, content = ? WHERE id = ?",
            title.unwrap_or(&self.title),
            content.unwrap_or(&self.content),
            self.id
        )
        .execute(database)
        .await
        .map(|_| ())
    }

    #[instrument(skip(database))]
    pub async fn delete(&self, database: &MySqlPool) -> Result<(), Error> {
        query!("DELETE FROM messages WHERE id = ?", self.id)
            .execute(database)
            .await
            .map(|_| ())
    }
}
