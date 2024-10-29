use std::sync::Arc;

use axum::{routing::get, Router};
use tracing::instrument;

use crate::{
    controllers::message::{create, destroy, events, index, show, update},
    services::state::StateService,
};

#[instrument(level = "debug")]
pub fn routes() -> Router<Arc<StateService>> {
    Router::default()
        .route("/messages", get(index).post(create))
        .route("/message/:id", get(show).put(update).delete(destroy))
        .route("/messages/events", get(events))
}
