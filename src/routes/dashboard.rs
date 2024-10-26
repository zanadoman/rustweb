use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::controllers::dashboard::index;

pub fn routes() -> Router<MySqlPool> {
    Router::default().route("/dashboard", get(index))
}
