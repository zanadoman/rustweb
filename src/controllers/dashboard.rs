use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tracing::instrument;

use crate::templates::dashbaord::DashboardTemplate;

#[instrument]
pub async fn index() -> impl IntoResponse {
    match (DashboardTemplate {}).render() {
        Ok(rendered) => (StatusCode::OK, Html(rendered)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
