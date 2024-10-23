use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::controllers::message::{create, destroy, index, show, update};

#[instrument]
pub fn routes() -> Router<Arc<MySqlPool>> {
    Router::default()
        .route("/messages", get(index).post(create))
        .route("/messages/:id", get(show).put(update).delete(destroy))
}
