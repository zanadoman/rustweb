use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::controllers::message::{create, destroy, index, show, update};

pub fn routes() -> Router<Arc<MySqlPool>> {
    Router::default()
        .route("/messages", get(index).post(create))
        .route("/messages/:id", get(show).put(update).delete(destroy))
}
