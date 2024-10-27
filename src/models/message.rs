use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query, query_as, Error, MySqlPool};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MessageModel {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

impl MessageModel {
    pub async fn find(
        database: &MySqlPool,
        id: i32,
    ) -> Result<Option<Self>, Error> {
        query_as!(Self, "SELECT * FROM messages WHERE id = ? LIMIT 1", id)
            .fetch_optional(database)
            .await
    }

    pub async fn all(database: &MySqlPool) -> Result<Vec<Self>, Error> {
        query_as!(Self, "SELECT * FROM messages")
            .fetch_all(database)
            .await
    }

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

    pub async fn update(
        database: &MySqlPool,
        id: i32,
        title: &String,
        content: &String,
    ) -> Result<(), Error> {
        query!(
            "UPDATE messages SET title = ?, content = ? WHERE id = ?",
            title,
            content,
            id,
        )
        .execute(database)
        .await
        .map(|_| ())
    }

    pub async fn delete(database: &MySqlPool, id: i32) -> Result<(), Error> {
        query!("DELETE FROM messages WHERE id = ?", id)
            .execute(database)
            .await
            .map(|_| ())
    }
}
