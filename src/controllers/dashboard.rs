use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_login::AuthSession;
use tracing::instrument;

use crate::{
    services::authenticator::AuthenticatorService,
    templates::dashboard::DashboardTemplate,
};

#[instrument(skip(authenticator))]
pub async fn index(
    authenticator: AuthSession<AuthenticatorService>,
) -> impl IntoResponse {
    match authenticator.user {
        Some(user) => match (DashboardTemplate {
            username: &user.name,
        })
        .render()
        {
            Ok(rendered) => (StatusCode::OK, Html(rendered)).into_response(),
            Err(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                    .into_response()
            }
        },
        None => Redirect::to("/authentication").into_response(),
    }
}
