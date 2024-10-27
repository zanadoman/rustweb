use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Extension, Form,
};
use axum_csrf::CsrfToken;
use http::{HeaderMap, StatusCode};
use sqlx::{Error, MySqlPool};
use tracing::{error, instrument, warn};

use crate::models::message::MessageModel;
use crate::templates::message::MessageTemplate;

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
    match (MessageTemplate {
        token: &token,
        message: &match MessageModel::find(&database, id).await {
            Ok(Some(message)) => message,
            Ok(None) => return StatusCode::NOT_FOUND.into_response(),
            Err(error) => {
                error!("{error}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        },
    })
    .render()
    {
        Ok(rendered) => (StatusCode::OK, csrf, Html(rendered)).into_response(),
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
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
    let mut messages = String::default();
    for message in match MessageModel::all(&database).await {
        Ok(messages) => messages,
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    } {
        match (MessageTemplate {
            token: &token,
            message: &message,
        })
        .render()
        {
            Ok(message) => messages.push_str(&message),
            Err(error) => {
                error!("{error}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
    (StatusCode::OK, csrf, Html(messages)).into_response()
}

#[instrument(skip(database))]
pub async fn create(
    State(database): State<MySqlPool>,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    match MessageModel::create(&database, &message.title, &message.content)
        .await
    {
        Ok(id) => {
            Redirect::to(format!("/message/{id}").as_str()).into_response()
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

#[instrument(skip(database))]
pub async fn update(
    Path(id): Path<i32>,
    State(database): State<MySqlPool>,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    match MessageModel::update(&database, id, &message.title, &message.content)
        .await
    {
        Ok(..) => {
            Redirect::to(format!("/message/{id}").as_str()).into_response()
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

#[instrument(skip(database))]
pub async fn destroy(
    Path(id): Path<i32>,
    State(database): State<MySqlPool>,
) -> impl IntoResponse {
    match MessageModel::delete(&database, id).await {
        Ok(..) => StatusCode::OK.into_response(),
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
