use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Extension, Form,
};
use axum_csrf::CsrfToken;
use http::{HeaderMap, StatusCode};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::models::message::MessageModel;
use crate::templates::{message::MessageTemplate, messages::MessagesTemplate};

#[instrument(skip(database, csrf))]
pub async fn show(
    Path(id): Path<i32>,
    State(database): State<MySqlPool>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers.get("Hx-Request").is_none() {
        return Redirect::to("/dashboard").into_response();
    }
    let message = match MessageModel::find(&database, id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match (MessageTemplate {
        token: &token,
        message: &message,
    })
    .render()
    {
        Ok(rendered) => (StatusCode::OK, csrf, Html(rendered)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(skip(database, csrf))]
pub async fn index(
    State(database): State<MySqlPool>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers.get("Hx-Request").is_none() {
        return Redirect::to("/dashboard").into_response();
    }
    let messages = match MessageModel::all(&database).await {
        Ok(messages) => messages,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match (MessagesTemplate {
        token: &token,
        messages: &messages,
    })
    .render()
    {
        Ok(rendered) => (StatusCode::OK, csrf, Html(rendered)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(skip(database))]
pub async fn create(
    State(database): State<MySqlPool>,
    Form(form): Form<MessageModel>,
) -> impl IntoResponse {
    match MessageModel::create(&database, &form.title, &form.content).await {
        Ok(id) => {
            Redirect::to(format!("/message/{}", id).as_str()).into_response()
        }
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

#[instrument(skip(database))]
pub async fn update(
    Path(id): Path<i32>,
    State(database): State<MySqlPool>,
    Form(form): Form<MessageModel>,
) -> impl IntoResponse {
    let message = match MessageModel::find(&database, id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match message
        .update(&database, Some(&form.title), Some(&form.content))
        .await
    {
        Ok(..) => {
            Redirect::to(format!("/message/{}", id).as_str()).into_response()
        }
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

#[instrument(skip(database))]
pub async fn destroy(
    Path(id): Path<i32>,
    State(database): State<MySqlPool>,
) -> impl IntoResponse {
    let message = match MessageModel::find(&database, id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match message.delete(&database).await {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}
