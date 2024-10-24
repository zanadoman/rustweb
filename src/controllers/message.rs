use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use sqlx::MySqlPool;
use tracing::instrument;

use crate::models::message::MessageModel;
use crate::templates::{message::MessageTemplate, messages::MessagesTemplate};

#[instrument(skip(database))]
pub async fn show(
    State(database): State<Arc<MySqlPool>>,
    Path(id): Path<i32>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers.get("Hx-Request").is_none() {
        return Redirect::to("/dashboard").into_response();
    }
    let message = match MessageModel::find(database.as_ref(), id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match (MessageTemplate { message: &message }).render() {
        Ok(rendered) => (StatusCode::OK, Html(rendered)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(skip(database))]
pub async fn index(
    State(database): State<Arc<MySqlPool>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers.get("Hx-Request").is_none() {
        return Redirect::to("/dashboard").into_response();
    }
    let messages = match MessageModel::all(database.as_ref()).await {
        Ok(messages) => messages,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match (MessagesTemplate {
        messages: &messages,
    })
    .render()
    {
        Ok(rendered) => (StatusCode::OK, Html(rendered)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(skip(database))]
pub async fn create(
    State(database): State<Arc<MySqlPool>>,
    Form(form): Form<MessageModel>,
) -> impl IntoResponse {
    match MessageModel::create(database.as_ref(), &form.title, &form.content)
        .await
    {
        Ok(id) => {
            Redirect::to(format!("/messages/{}", id).as_str()).into_response()
        }
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

#[instrument(skip(database))]
pub async fn update(
    State(database): State<Arc<MySqlPool>>,
    Path(id): Path<i32>,
    Form(form): Form<MessageModel>,
) -> impl IntoResponse {
    let message = match MessageModel::find(database.as_ref(), id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match message
        .update(database.as_ref(), Some(&form.title), Some(&form.content))
        .await
    {
        Ok(..) => {
            Redirect::to(format!("/messages/{}", id).as_str()).into_response()
        }
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

#[instrument(skip(database))]
pub async fn destroy(
    State(database): State<Arc<MySqlPool>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let message = match MessageModel::find(database.as_ref(), id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                .into_response()
        }
    };
    match message.delete(database.as_ref()).await {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}
