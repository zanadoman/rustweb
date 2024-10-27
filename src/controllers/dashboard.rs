use std::sync::Arc;

use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use http::StatusCode;
use tracing::{error, instrument};

use crate::{
    services::authenticator::AuthenticatorService,
    templates::dashboard::DashboardTemplate,
};

#[instrument(level = "debug", skip(authenticator, csrf))]
pub async fn index(
    authenticator: AuthSession<AuthenticatorService>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
) -> impl IntoResponse {
    match (DashboardTemplate {
        token: &token,
        location: "Dashboard",
        username: &match authenticator.user {
            Some(user) => user.name,
            None => {
                return (StatusCode::FOUND, [("HX-Location", "/")])
                    .into_response()
            }
        },
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
