use std::sync::Arc;

use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::controllers::authentication::{
    authentication, login, logout, register,
};

#[instrument]
pub fn routes() -> Router<Arc<MySqlPool>> {
    Router::default()
        .route("/authentication", get(authentication))
        .route(
            "/register",
            get(|| async { Redirect::to("/") }).post(register),
        )
        .route("/login", get(|| async { Redirect::to("/") }).post(login))
        .route("/logout", post(logout))
}
