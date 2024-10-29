use std::sync::Arc;

use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_csrf::CsrfToken;
use tracing::{error, instrument, warn};

#[instrument(level = "trace")]
pub async fn csrf_provider(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    if request.method() != Method::GET {
        return Ok(next.run(request).await);
    }
    let Some(csrf) = request.extensions().get::<CsrfToken>() else {
        error!("missing CsrfToken extension");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };
    let token = csrf.authenticity_token().map_err(|error| {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    if request.extensions_mut().insert(Arc::new(token)).is_some() {
        error!("token insertion failed");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
    Ok(next.run(request).await)
}

#[instrument(level = "trace")]
pub async fn csrf_verifier(
    request: Request<Body>,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    if !matches!(
        *request.method(),
        Method::POST | Method::PUT | Method::DELETE | Method::PATCH
    ) {
        return Ok(next.run(request).await);
    }
    let Some(csrf) = request.extensions().get::<CsrfToken>() else {
        error!("missing CsrfToken extension");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };
    let Some(token) = request.headers().get("X-CSRF-Token") else {
        warn!("missing X-CSRF-Token header");
        return Err((StatusCode::FORBIDDEN, "Missing Token.").into_response());
    };
    let token = token.to_str().map_err(|error| {
        warn!("{error}");
        (StatusCode::FORBIDDEN, "Invalid Token.").into_response()
    })?;
    if let Err(error) = csrf.verify(token) {
        warn!("{error}");
        return Err((StatusCode::FORBIDDEN, error.to_string()).into_response());
    }
    Ok(next.run(request).await)
}
