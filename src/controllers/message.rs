use std::{error::Error, sync::Arc};

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
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
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
    let message = match MessageModel::find(&state.database(), id).await {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    match (MessageTemplate {
        token: &token,
        message: &message,
    })
    .render()
    {
        Ok(message) => (StatusCode::OK, csrf, Html(message)).into_response(),
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
    let id = match MessageModel::create(
        &state.database(),
        &message.title,
        &message.content,
    )
    .await
    {
        Ok(query) => query.last_insert_id(),
        Err(sqlx::Error::Database(error)) => {
            warn!("{error}");
            return (StatusCode::CONFLICT, error.to_string()).into_response();
        }
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let id = match i32::try_from(id) {
        Ok(id) => id,
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    if let Err(error) = state.messages().send((
        Event::default()
            .id(state.id().to_string())
            .event("messages"),
        Some(MessageModel {
            id: Some(id),
            title: message.title,
            content: message.content,
        }),
    )) {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::NO_CONTENT.into_response()
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
        Err(sqlx::Error::Database(error)) => {
            warn!("{error}");
            return (StatusCode::CONFLICT, error.to_string()).into_response();
        }
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        _ => (),
    }
    if let Err(error) = state.messages().send((
        Event::default()
            .id(state.id().to_string())
            .event(format!("message{id}")),
        Some(MessageModel {
            id: Some(id),
            title: message.title,
            content: message.content,
        }),
    )) {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::NO_CONTENT.into_response()
    }
}

#[instrument(level = "debug")]
pub async fn destroy(
    Path(id): Path<i32>,
    State(state): State<Arc<StateService>>,
) -> impl IntoResponse {
    match MessageModel::delete(&state.database(), id).await {
        Err(sqlx::Error::Database(error)) => {
            warn!("{error}");
            return (StatusCode::CONFLICT, error.to_string()).into_response();
        }
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        _ => (),
    };
    if let Err(error) = state.messages().send((
        Event::default()
            .id(state.id().to_string())
            .event(format!("message{id}")),
        None,
    )) {
        error!("{error}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::NO_CONTENT.into_response()
    }
}

pub async fn events(
    State(state): State<Arc<StateService>>,
    Extension(token): Extension<Arc<String>>,
) -> Sse<impl Stream<Item = Result<Event, Box<dyn Error + Send + Sync>>>> {
    Sse::new(BroadcastStream::new(state.messages().subscribe()).map(
        move |event| match event {
            Ok((event, message)) => {
                Ok(event.data(if let Some(message) = message {
                    match (MessageTemplate {
                        token: &token,
                        message: &message,
                    })
                    .render()
                    {
                        Ok(message) => message,
                        Err(error) => {
                            error!("{error}");
                            return Err(
                                Box::new(error) as Box<dyn Error + Send + Sync>
                            );
                        }
                    }
                } else {
                    String::default()
                }))
            }
            Err(error) => {
                error!("{error}");
                Err(Box::new(error) as Box<dyn Error + Send + Sync>)
            }
        },
    ))
    .keep_alive(KeepAlive::default())
}
