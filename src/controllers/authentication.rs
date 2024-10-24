use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_login::AuthSession;
use sqlx::MySqlPool;
use tracing::instrument;

use crate::{
    models::user::UserModel, services::authenticator::AuthenticatorService,
    templates::authentication::AuthenticationTemplate,
};

#[instrument]
pub async fn authentication() -> impl IntoResponse {
    match (AuthenticationTemplate {}).render() {
        Ok(rendered) => (StatusCode::OK, Html(rendered)).into_response(),
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
        Ok(..) => Redirect::to("/login").into_response(),
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
            Ok(..) => Redirect::to("/dashboard").into_response(),
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
        Ok(..) => Redirect::to("/authentication").into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
