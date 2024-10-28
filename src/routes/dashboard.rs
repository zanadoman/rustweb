use axum::{routing::get, Router};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::controllers::dashboard::index;

#[instrument(level = "debug")]
pub fn routes() -> Router<MySqlPool> {
    Router::default().route("/dashboard", get(index))
}
