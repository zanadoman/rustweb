use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tracing::instrument;

use crate::{
    controllers::authentication::{
        authentication, login, logout, register, validate_name,
        validate_password,
    },
    services::state::StateService,
};

#[instrument(level = "debug")]
pub fn routes() -> Router<Arc<StateService>> {
    Router::new()
        .route("/", get(authentication))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register/validate/name", post(validate_name))
        .route("/register/validate/password", post(validate_password))
}
