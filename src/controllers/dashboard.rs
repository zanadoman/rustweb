use std::sync::Arc;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
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
    let Some(user) = authenticator.user else {
        return (StatusCode::SEE_OTHER, csrf, [("HX-Location", "/")])
            .into_response();
    };
    match (DashboardTemplate {
        token: &token,
        location: "Dashboard",
        name: Some(&user.name),
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
