use axum::async_trait;
use axum_login::{
    tower_sessions::{Expiry, SessionManagerLayer},
    AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
};
use password_auth::verify_password;
use sqlx::{Error, MySqlPool};
use time::Duration;
use tower_sessions_sqlx_store::MySqlStore;

use crate::models::user::UserModel;

#[derive(Clone)]
pub struct AuthenticatorService(MySqlPool);

#[async_trait]
impl AuthnBackend for AuthenticatorService {
    type User = UserModel;
    type Credentials = UserModel;
    type Error = Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.get_user(&credentials.name).await?.filter(|user| {
            verify_password(credentials.password, &user.password).is_ok()
        }))
    }

    async fn get_user(
        &self,
        name: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        UserModel::find(&self.0, name).await
    }
}

impl AuthenticatorService {
    pub async fn new(
        database: &MySqlPool,
    ) -> Result<AuthManagerLayer<AuthenticatorService, MySqlStore>, Error> {
        let storage = MySqlStore::new(database.clone());
        storage.migrate().await?;
        Ok(AuthManagerLayerBuilder::new(
            AuthenticatorService {
                0: database.clone(),
            },
            SessionManagerLayer::new(storage)
                .with_expiry(Expiry::OnInactivity(Duration::minutes(10)))
                .with_secure(false),
        )
        .build())
    }
}
