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
    templates::{
        authentication::{
            AuthenticationFormNameTemplate, AuthenticationFormPasswordTemplate,
            AuthenticationLoginTemplate, AuthenticationTemplate,
        },
        toast::ToastTemplate,
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
        name: None,
        form_name: &AuthenticationFormNameTemplate {
            token: &token,
            value: "",
            error: None,
        },
        form_password: &AuthenticationFormPasswordTemplate {
            token: &token,
            value: "",
            error: None,
        },
    })
    .render()
    {
        Ok(authentication) => (
            StatusCode::OK,
            [("HX-Retarget", "body")],
            csrf,
            Html(authentication),
        )
            .into_response(),
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
    match UserModel::create(state.database(), &user.name, &user.password).await
    {
        Err(Error::Database(error)) => {
            warn!("{error}");
            return (StatusCode::CONFLICT, csrf, error.to_string())
                .into_response();
        }
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        _ => (),
    }
    match (ToastTemplate {
        content: "Successful registration.",
    })
    .render()
    {
        Ok(toast) => (StatusCode::CREATED, csrf, Html(toast)).into_response(),
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(authenticator, csrf))]
pub async fn login(
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    mut authenticator: AuthSession<AuthenticatorService>,
    Form(user): Form<UserModel>,
) -> impl IntoResponse {
    let user = match authenticator.authenticate(user).await {
        Ok(user) => user,
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    if let Some(user) = user {
        if let Err(error) = authenticator.login(&user).await {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        } else {
            (StatusCode::SEE_OTHER, [("HX-Location", "/dashboard")], csrf)
                .into_response()
        }
    } else {
        match (AuthenticationLoginTemplate {
            token: &token,
            error: true,
        })
        .render()
        {
            Ok(login) => (StatusCode::OK, csrf, Html(login)).into_response(),
            Err(error) => {
                error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[instrument(level = "debug", skip(authenticator, csrf))]
pub async fn logout(
    csrf: CsrfToken,
    mut authenticator: AuthSession<AuthenticatorService>,
) -> impl IntoResponse {
    if let Err(error) = authenticator.logout().await {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        (StatusCode::SEE_OTHER, [("HX-Location", "/")], csrf).into_response()
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
        error: UserModel::validate_name(state.database(), &user.name).await,
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
        error: UserModel::validate_password(&user.password),
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
