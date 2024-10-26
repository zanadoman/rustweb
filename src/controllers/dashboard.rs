use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use tracing::instrument;

use crate::{
    services::authenticator::AuthenticatorService,
    templates::dashboard::DashboardTemplate,
};

#[instrument(skip(csrf, authenticator))]
pub async fn index(
    csrf: CsrfToken,
    authenticator: AuthSession<AuthenticatorService>,
) -> impl IntoResponse {
    match authenticator.user {
        Some(user) => match (DashboardTemplate {
            token: match csrf.authenticity_token() {
                Ok(token) => token,
                Err(error) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        error.to_string(),
                    )
                        .into_response()
                }
            },
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
