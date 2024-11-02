use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use sqlx::Error;
use tracing::{error, instrument, warn};

use crate::{
    models::user::UserModel,
    services::{authenticator::AuthenticatorService, state::StateService},
    templates::authentication::AuthenticationTemplate,
};

#[instrument(level = "debug", skip(csrf))]
pub async fn authentication(
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
) -> impl IntoResponse {
    match (AuthenticationTemplate {
        token: &token,
        location: "Authentication",
    })
    .render()
    {
        Ok(authentication) => {
            (StatusCode::OK, csrf, Html(authentication)).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug")]
pub async fn register(
    State(state): State<Arc<StateService>>,
    Form(form): Form<UserModel>,
) -> impl IntoResponse {
    match UserModel::create(&state.database(), &form.name, &form.password).await
    {
        Ok(..) => {
            (StatusCode::SEE_OTHER, [("HX-Location", "/")]).into_response()
        }
        Err(Error::Database(error)) => {
            warn!("{error}");
            (StatusCode::CONFLICT, error.to_string()).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(authenticator))]
pub async fn login(
    mut authenticator: AuthSession<AuthenticatorService>,
    Form(form): Form<UserModel>,
) -> impl IntoResponse {
    let user = match authenticator.authenticate(form).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    if let Err(error) = authenticator.login(&user).await {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        (StatusCode::SEE_OTHER, [("HX-Location", "/dashboard")]).into_response()
    }
}

#[instrument(level = "debug", skip(authenticator))]
pub async fn logout(
    mut authenticator: AuthSession<AuthenticatorService>,
) -> impl IntoResponse {
    if let Err(error) = authenticator.logout().await {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        (StatusCode::SEE_OTHER, [("HX-Location", "/")]).into_response()
    }
}
