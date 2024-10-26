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
    if let Err(error) = csrf.verify(&String::from_utf8_lossy(token.as_ref())) {
        warn!("{error}");
        return Err((StatusCode::FORBIDDEN, error.to_string()).into_response());
    }
    Ok(next.run(request).await)
}
