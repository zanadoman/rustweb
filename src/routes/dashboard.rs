use std::sync::Arc;

use axum::{routing::get, Router};
use tracing::instrument;

use crate::{controllers::dashboard::index, services::state::StateService};

#[instrument(level = "debug")]
pub fn routes() -> Router<Arc<StateService>> {
    Router::default().route("/dashboard", get(index))
}
