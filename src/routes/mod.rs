mod authentication;
mod dashboard;
mod message;

use std::sync::Arc;

use axum::Router;
use axum_login::login_required;
use tracing::instrument;

use crate::services::{
    authenticator::AuthenticatorService, state::StateService,
};

#[instrument(level = "debug")]
pub fn routes() -> Router<Arc<StateService>> {
    message::routes()
        .merge(dashboard::routes())
        .route_layer(login_required!(AuthenticatorService, login_url = "/"))
        .merge(authentication::routes())
}
