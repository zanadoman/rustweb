use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::controllers::authentication::{
    authentication, login, logout, register,
};

pub fn routes() -> Router<Arc<MySqlPool>> {
    Router::default()
        .route("/", get(authentication))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}
