use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tracing::instrument;

use crate::{
    controllers::authentication::{authentication, login, logout, register},
    services::state::StateService,
};

#[instrument(level = "debug")]
pub fn routes() -> Router<Arc<StateService>> {
    Router::default()
        .route("/", get(authentication))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}
