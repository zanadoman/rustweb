use std::{fmt::Debug, result::Result};

use axum_login::AuthUser;
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query, query_as, Error, MySqlPool};

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UserModel {
    pub name: String,
    pub password: String,
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
    pub async fn find(
        database: &MySqlPool,
        name: &String,
    ) -> Result<Option<Self>, Error> {
        query_as!(
            UserModel,
            "SELECT * FROM users WHERE name = ? LIMIT 1",
            name
        )
        .fetch_optional(database)
        .await
    }

    pub async fn create(
        database: &MySqlPool,
        name: &String,
        password: &String,
    ) -> Result<(), Error> {
        query!(
            "INSERT INTO users (name, password) VALUES (?, ?)",
            name,
            generate_hash(password)
        )
        .execute(database)
        .await
        .map(|_| ())
    }
}
