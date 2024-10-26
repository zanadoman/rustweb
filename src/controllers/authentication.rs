use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use sqlx::MySqlPool;
use tracing::instrument;

use crate::{
    models::user::UserModel, services::authenticator::AuthenticatorService,
    templates::authentication::AuthenticationTemplate,
};

#[instrument(skip(csrf))]
pub async fn authentication(csrf: CsrfToken) -> impl IntoResponse {
    match (AuthenticationTemplate {
        token: match csrf.authenticity_token() {
            Ok(token) => token,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                    .into_response()
            }
        },
        location: "Authentication",
    })
    .render()
    {
        Ok(rendered) => (StatusCode::OK, csrf, Html(rendered)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(skip(database))]
pub async fn register(
    State(database): State<Arc<MySqlPool>>,
    Form(form): Form<UserModel>,
) -> impl IntoResponse {
    match UserModel::create(database.as_ref(), &form.name, &form.password).await
    {
        Ok(..) => (StatusCode::FOUND, [("HX-Location", "/")]).into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

#[instrument(skip(authenticator))]
pub async fn login(
    mut authenticator: AuthSession<AuthenticatorService>,
    Form(form): Form<UserModel>,
) -> impl IntoResponse {
    match authenticator.authenticate(form).await {
        Ok(Some(user)) => match authenticator.login(&user).await {
            Ok(..) => (StatusCode::FOUND, [("HX-Location", "/dashboard")])
                .into_response(),
            Err(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                    .into_response()
            }
        },
        Ok(None) => StatusCode::UNAUTHORIZED.into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(skip(authenticator))]
pub async fn logout(
    mut authenticator: AuthSession<AuthenticatorService>,
) -> impl IntoResponse {
    match authenticator.logout().await {
        Ok(..) => (StatusCode::FOUND, [("HX-Location", "/")]).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
