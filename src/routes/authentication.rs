use axum::{
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::controllers::authentication::{
    authentication, login, logout, register,
};

#[instrument(level = "debug")]
pub fn routes() -> Router<MySqlPool> {
    Router::default()
        .route("/", get(authentication))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}
