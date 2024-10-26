use std::sync::Arc;

use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use http::StatusCode;
use tracing::instrument;

use crate::{
    services::authenticator::AuthenticatorService,
    templates::dashboard::DashboardTemplate,
};

#[instrument(skip(csrf, authenticator))]
pub async fn index(
    authenticator: AuthSession<AuthenticatorService>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
) -> impl IntoResponse {
    match authenticator.user {
        Some(user) => match (DashboardTemplate {
            token: &token,
            location: "Dashboard",
            username: &user.name,
        })
        .render()
        {
            Ok(rendered) => {
                (StatusCode::OK, csrf, Html(rendered)).into_response()
            }
            Err(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                    .into_response()
            }
        },
        None => (StatusCode::FOUND, [("Location", "/")]).into_response(),
    }
}
