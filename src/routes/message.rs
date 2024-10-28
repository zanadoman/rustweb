use axum::{routing::get, Router};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::controllers::message::{create, destroy, index, show, update};

#[instrument(level = "debug")]
pub fn routes() -> Router<MySqlPool> {
    Router::default()
        .route("/messages", get(index).post(create))
        .route("/message/:id", get(show).put(update).delete(destroy))
}
