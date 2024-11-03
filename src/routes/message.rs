use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tracing::instrument;

use crate::{
    controllers::message::{
        create, destroy, events, index, show, update, validate_content,
        validate_title,
    },
    services::state::StateService,
};

#[instrument(level = "debug")]
pub fn routes() -> Router<Arc<StateService>> {
    Router::new()
        .route("/messages", get(index).post(create))
        .route("/message/:id", get(show).put(update).delete(destroy))
        .route("/messages/events", get(events))
        .route("/message/:id/validate/title", post(validate_title))
        .route("/message/:id/validate/content", post(validate_content))
}
