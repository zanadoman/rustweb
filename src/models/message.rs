use serde::{Deserialize, Serialize};
use sqlx::{
    mysql::MySqlQueryResult, query, query_as, Error, FromRow, MySqlPool,
};
use tracing::instrument;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct MessageModel {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct MessageModelError {
    pub title: Option<&'static str>,
    pub content: Option<&'static str>,
}

impl MessageModel {
    #[instrument(level = "trace")]
    pub async fn find(
        database: &MySqlPool,
        id: i32,
    ) -> Result<Option<Self>, Error> {
        query_as!(Self, "SELECT * FROM messages WHERE id = ? LIMIT 1;", id)
            .fetch_optional(database)
            .await
    }

    #[instrument(level = "trace")]
    pub async fn all(database: &MySqlPool) -> Result<Vec<Self>, Error> {
        query_as!(Self, "SELECT * FROM messages ORDER BY id DESC;")
            .fetch_all(database)
            .await
    }

    #[instrument(level = "trace")]
    pub async fn create(
        database: &MySqlPool,
        title: &str,
        content: &str,
    ) -> Result<MySqlQueryResult, Error> {
        query!(
            "INSERT INTO messages (title, content) VALUES (?, ?);",
            title,
            content
        )
        .execute(database)
        .await
    }

    #[instrument(level = "trace")]
    pub async fn update(
        database: &MySqlPool,
        id: i32,
        title: &str,
        content: &str,
    ) -> Result<MySqlQueryResult, Error> {
        query!(
            "UPDATE messages SET title = ?, content = ? WHERE id = ?;",
            title,
            content,
            id
        )
        .execute(database)
        .await
    }

    #[instrument(level = "trace")]
    pub async fn delete(
        database: &MySqlPool,
        id: i32,
    ) -> Result<MySqlQueryResult, Error> {
        query!("DELETE FROM messages WHERE id = ?;", id)
            .execute(database)
            .await
    }

    #[instrument(level = "trace")]
    pub fn validate_title(title: &str) -> Option<&'static str> {
        if title.is_empty() {
            Some("Title must be at least 1 character long.")
        } else if 100 < title.len() {
            Some("Title must not be more than 100 characters long.")
        } else {
            None
        }
    }

    #[instrument(level = "trace")]
    pub fn validate_content(content: &str) -> Option<&'static str> {
        if content.is_empty() {
            Some("Content must be at least 1 character long.")
        } else if 100 < content.len() {
            Some("Content must not be more than 1000 characters long.")
        } else {
            None
        }
    }

    #[instrument(level = "trace")]
    pub fn validate(message: &Self) -> Option<MessageModelError> {
        let title = Self::validate_title(&message.title);
        let content = Self::validate_content(&message.content);
        if title.is_some() || content.is_some() {
            Some(MessageModelError { title, content })
        } else {
            None
        }
    }
}
