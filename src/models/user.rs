use std::{
    fmt::{self, Debug, Formatter},
    result::Result,
};

use axum_login::AuthUser;
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use sqlx::{
    mysql::MySqlQueryResult, query, query_as, Error, FromRow, MySqlPool,
};
use tracing::instrument;

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct UserModel {
    pub name: String,
    pub password: String,
}

impl Debug for UserModel {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UserModel")
            .field("name", &self.name)
            .field("password", &"********")
            .finish()
    }
}

impl AuthUser for UserModel {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

impl UserModel {
    #[instrument(level = "trace")]
    pub async fn find(
        database: &MySqlPool,
        name: &str,
    ) -> Result<Option<Self>, Error> {
        query_as!(Self, "SELECT * FROM users WHERE name = ? LIMIT 1", name)
            .fetch_optional(database)
            .await
    }

    #[instrument(level = "trace")]
    pub async fn create(
        database: &MySqlPool,
        name: &str,
        password: &str,
    ) -> Result<MySqlQueryResult, Error> {
        query!(
            "INSERT INTO users VALUES (?, ?)",
            name,
            generate_hash(password)
        )
        .execute(database)
        .await
    }
}
