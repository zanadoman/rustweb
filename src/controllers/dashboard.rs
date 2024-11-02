use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use tracing::{error, instrument};

use crate::{
    models::message::MessageModel,
    services::{authenticator::AuthenticatorService, state::StateService},
    templates::dashboard::DashboardTemplate,
};

#[instrument(level = "debug", skip(authenticator, csrf))]
pub async fn index(
    State(state): State<Arc<StateService>>,
    authenticator: AuthSession<AuthenticatorService>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
) -> impl IntoResponse {
    let Some(user) = authenticator.user else {
        return (StatusCode::SEE_OTHER, [("HX-Location", "/")]).into_response();
    };
    let messages = match MessageModel::all(state.database()).await {
        Ok(messages) => messages,
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    match (DashboardTemplate {
        token: &token,
        location: "Dashboard",
        username: &user.name,
        messages: &messages,
    })
    .render()
    {
        Ok(dashboard) => {
            (StatusCode::OK, csrf, Html(dashboard)).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
