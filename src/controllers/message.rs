use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{
        sse::{Event, KeepAlive, Sse},
        Html, IntoResponse, Redirect,
    },
    Extension, Form,
};
use axum_csrf::CsrfToken;
use sqlx::Error;
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    Stream,
};
use tracing::{error, instrument, warn};

use crate::templates::message::MessageTemplate;
use crate::{models::message::MessageModel, services::state::StateService};

#[instrument(level = "debug", skip(csrf))]
pub async fn show(
    Path(id): Path<i32>,
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers.get("Hx-Request").is_none() {
        return Redirect::to("/dashboard").into_response();
    }
    match (MessageTemplate {
        token: &token,
        message: &match MessageModel::find(&state.database(), id).await {
            Ok(Some(message)) => message,
            Ok(None) => return StatusCode::RESET_CONTENT.into_response(),
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

#[instrument(level = "debug", skip(csrf))]
pub async fn index(
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers.get("Hx-Request").is_none() {
        return Redirect::to("/dashboard").into_response();
    }
    let mut messages = String::default();
    for message in match MessageModel::all(&state.database()).await {
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

#[instrument(level = "debug")]
pub async fn create(
    State(state): State<Arc<StateService>>,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    match MessageModel::create(
        &state.database(),
        &message.title,
        &message.content,
    )
    .await
    {
        Ok(..) => {
            if let Err(error) = state.messages().send(
                Event::default()
                    .id(state.id().to_string())
                    .event("messages")
                    .data("Message created."),
            ) {
                error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                StatusCode::NO_CONTENT.into_response()
            }
        }
        .into_response(),
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

#[instrument(level = "debug")]
pub async fn update(
    Path(id): Path<i32>,
    State(state): State<Arc<StateService>>,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    match MessageModel::update(
        &state.database(),
        id,
        &message.title,
        &message.content,
    )
    .await
    {
        Ok(..) => {
            if let Err(error) = state.messages().send(
                Event::default()
                    .id(state.id().to_string())
                    .event(format!("message{id}"))
                    .data("Message updated."),
            ) {
                error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                StatusCode::NO_CONTENT.into_response()
            }
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

#[instrument(level = "debug")]
pub async fn destroy(
    Path(id): Path<i32>,
    State(state): State<Arc<StateService>>,
) -> impl IntoResponse {
    match MessageModel::delete(&state.database(), id).await {
        Ok(..) => {
            if let Err(error) = state.messages().send(
                Event::default()
                    .id(state.id().to_string())
                    .event(format!("message{id}"))
                    .data("Message destroyed."),
            ) {
                error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                StatusCode::NO_CONTENT.into_response()
            }
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

#[instrument(level = "debug")]
pub async fn events(
    State(state): State<Arc<StateService>>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    Sse::new(BroadcastStream::new(state.messages().subscribe()))
        .keep_alive(KeepAlive::default())
}
