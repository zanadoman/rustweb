use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::controllers::index::index;

pub fn routes() -> Router<Arc<MySqlPool>> {
    Router::default().route("/", get(index))
}
