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
use tracing::{error, instrument};

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct UserModel {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserModelError {
    pub name: Option<String>,
    pub password: Option<String>,
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
        query_as!(Self, "SELECT * FROM users WHERE name = ? LIMIT 1;", name)
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
            "INSERT INTO users VALUES (?, ?);",
            name,
            generate_hash(password)
        )
        .execute(database)
        .await
    }

    #[instrument(level = "trace")]
    pub async fn validate_name(
        database: &MySqlPool,
        name: &str,
    ) -> Option<String> {
        if name.is_empty() {
            return Some("Name must be at least 1 character long.".to_owned());
        }
        if 100 < name.len() {
            return Some(
                "Name must not be more than 50 characters long.".to_owned(),
            );
        }
        match query_as!(
            Self,
            "SELECT * FROM users WHERE name = ? LIMIT 1;",
            name
        )
        .fetch_optional(database)
        .await
        {
            Ok(Some(..)) => Some("Name already taken.".to_owned()),
            Ok(None) => None,
            Err(error) => {
                error!("{error}");
                Some("Internal server error.".to_owned())
            }
        }
    }

    #[instrument(level = "trace")]
    pub fn validate_password(password: &str) -> Option<String> {
        if password.len() < 8 {
            Some("Password must be at least 8 characters long.".to_owned())
        } else {
            None
        }
    }

    #[instrument(level = "trace")]
    pub async fn validate(
        database: &MySqlPool,
        user: &Self,
    ) -> Option<UserModelError> {
        let name = Self::validate_name(database, &user.name).await;
        let password = Self::validate_password(&user.password);
        if name.is_some() || password.is_some() {
            Some(UserModelError { name, password })
        } else {
            None
        }
    }
}
