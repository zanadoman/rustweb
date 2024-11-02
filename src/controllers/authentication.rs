use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Form, Json,
};
use axum_csrf::CsrfToken;
use axum_login::AuthSession;
use sqlx::Error;
use tracing::{error, instrument, warn};

use crate::{
    models::user::UserModel,
    services::{authenticator::AuthenticatorService, state::StateService},
    templates::authentication::{
        AuthenticationFormNameTemplate, AuthenticationFormPasswordTemplate,
        AuthenticationTemplate,
    },
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

#[instrument(level = "debug", skip(csrf))]
pub async fn register(
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Form(user): Form<UserModel>,
) -> impl IntoResponse {
    if let Some(error) = UserModel::validate(state.database(), &user).await {
        return (StatusCode::BAD_REQUEST, csrf, Json(error)).into_response();
    }
    match UserModel::create(&state.database(), &user.name, &user.password).await
    {
        Ok(..) => (StatusCode::NO_CONTENT, csrf).into_response(),
        Err(Error::Database(error)) => {
            warn!("{error}");
            (StatusCode::CONFLICT, csrf, error.to_string()).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(authenticator, csrf))]
pub async fn login(
    mut authenticator: AuthSession<AuthenticatorService>,
    csrf: CsrfToken,
    Form(user): Form<UserModel>,
) -> impl IntoResponse {
    let user = match authenticator.authenticate(user).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, csrf).into_response(),
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    if let Err(error) = authenticator.login(&user).await {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        (StatusCode::SEE_OTHER, csrf, [("HX-Location", "/dashboard")])
            .into_response()
    }
}

#[instrument(level = "debug", skip(authenticator, csrf))]
pub async fn logout(
    mut authenticator: AuthSession<AuthenticatorService>,
    csrf: CsrfToken,
) -> impl IntoResponse {
    if let Err(error) = authenticator.logout().await {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        (StatusCode::SEE_OTHER, csrf, [("HX-Location", "/")]).into_response()
    }
}

#[instrument(level = "debug", skip(csrf))]
pub async fn validate_name(
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    Form(user): Form<UserModel>,
) -> impl IntoResponse {
    match (AuthenticationFormNameTemplate {
        token: &token,
        value: &user.name,
        error: UserModel::validate_name(state.database(), &user.name)
            .await
            .as_deref(),
    })
    .render()
    {
        Ok(form_name) => {
            (StatusCode::OK, csrf, Html(form_name)).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(csrf))]
pub async fn validate_password(
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    Form(user): Form<UserModel>,
) -> impl IntoResponse {
    match (AuthenticationFormPasswordTemplate {
        token: &token,
        value: &user.password,
        error: UserModel::validate_password(&user.password).as_deref(),
    })
    .render()
    {
        Ok(form_password) => {
            (StatusCode::OK, csrf, Html(form_password)).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
