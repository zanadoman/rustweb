use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::controllers::dashboard::index;

#[instrument]
pub fn routes() -> Router<Arc<MySqlPool>> {
    Router::default().route("/dashboard", get(index))
}
