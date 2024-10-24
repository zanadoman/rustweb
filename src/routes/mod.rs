mod authentication;
mod dashboard;
mod message;

use std::sync::Arc;

use axum::Router;
use axum_login::login_required;
use sqlx::MySqlPool;

use crate::services::authenticator::AuthenticatorService;

pub fn routes() -> Router<Arc<MySqlPool>> {
    message::routes()
        .merge(dashboard::routes())
        .route_layer(login_required!(AuthenticatorService, login_url = "/"))
        .merge(authentication::routes())
}
