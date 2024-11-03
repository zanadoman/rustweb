use std::{error::Error, sync::Arc};

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{
        sse::{Event, KeepAlive},
        Html, IntoResponse, Redirect, Sse,
    },
    Extension, Form, Json,
};
use axum_csrf::CsrfToken;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use tracing::{error, instrument, warn};

use crate::{
    models::message::MessageModel,
    services::state::StateService,
    templates::{
        message::{
            MessageEventTemplate, MessageFormContentTemplate,
            MessageFormTitleTemplate, MessageIndexTemplate,
            MessageShowTemplate,
        },
        toast::ToastTemplate,
    },
};

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
    let message = match MessageModel::find(state.database(), id).await {
        Ok(Some(message)) => message,
        Ok(None) => return (StatusCode::NOT_FOUND, csrf).into_response(),
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    match (MessageShowTemplate {
        token: &token,
        message: &message,
    })
    .render()
    {
        Ok(show) => (StatusCode::OK, csrf, Html(show)).into_response(),
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
    let messages = match MessageModel::all(state.database()).await {
        Ok(messages) => messages,
        Err(error) => {
            error!("{error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    match (MessageIndexTemplate {
        token: &token,
        messages: &messages,
    })
    .render()
    {
        Ok(index) => (StatusCode::OK, csrf, Html(index)).into_response(),
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(csrf))]
pub async fn create(
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    if let Some(error) = MessageModel::validate(&message) {
        return (StatusCode::BAD_REQUEST, csrf, Json(error)).into_response();
    }
    let id = match MessageModel::create(
        state.database(),
        &message.title,
        &message.content,
    )
    .await
    {
        Ok(query) => query.last_insert_id(),
        Err(sqlx::Error::Database(error)) => {
            warn!("{error}");
            return (StatusCode::CONFLICT, csrf, error.to_string())
                .into_response();
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
        Event::default().id(state.id().to_string()).event("create"),
        Some(MessageModel {
            id: Some(id),
            title: message.title,
            content: message.content,
        }),
    )) {
        error!("{error}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    match (ToastTemplate {
        content: &format!("Message #{id} sent."),
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

#[instrument(level = "debug", skip(csrf))]
pub async fn update(
    Path(id): Path<i32>,
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    if let Some(error) = MessageModel::validate(&message) {
        return (StatusCode::BAD_REQUEST, csrf, Json(error)).into_response();
    }
    match MessageModel::update(
        state.database(),
        id,
        &message.title,
        &message.content,
    )
    .await
    {
        Err(sqlx::Error::Database(error)) => {
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
    if let Err(error) = state.messages().send((
        Event::default()
            .id(state.id().to_string())
            .event(format!("update{id}")),
        Some(MessageModel {
            id: Some(id),
            title: message.title,
            content: message.content,
        }),
    )) {
        error!("{error}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    match (ToastTemplate {
        content: &format!("Message #{id} edited."),
    })
    .render()
    {
        Ok(toast) => (StatusCode::OK, csrf, Html(toast)).into_response(),
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(csrf))]
pub async fn destroy(
    Path(id): Path<i32>,
    State(state): State<Arc<StateService>>,
    csrf: CsrfToken,
) -> impl IntoResponse {
    match MessageModel::delete(state.database(), id).await {
        Err(sqlx::Error::Database(error)) => {
            warn!("{error}");
            return (StatusCode::CONFLICT, csrf, error.to_string())
                .into_response();
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
            .event(format!("destroy{id}")),
        None,
    )) {
        error!("{error}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    match (ToastTemplate {
        content: &format!("Message #{id} deleted."),
    })
    .render()
    {
        Ok(toast) => (StatusCode::OK, csrf, Html(toast)).into_response(),
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug")]
pub async fn events(
    State(state): State<Arc<StateService>>,
    Extension(token): Extension<Arc<String>>,
) -> Sse<impl Stream<Item = Result<Event, Box<dyn Error + Send + Sync>>>> {
    Sse::new(BroadcastStream::new(state.messages().subscribe()).map(
        move |event| match event {
            Ok((event, message)) => {
                Ok(event.data(if let Some(message) = message {
                    match (MessageEventTemplate {
                        token: &token,
                        message: &message,
                    })
                    .render()
                    {
                        Ok(event) => event,
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

#[instrument(level = "debug", skip(csrf))]
pub async fn validate_title(
    Path(id): Path<i32>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    match (MessageFormTitleTemplate {
        token: &token,
        id,
        value: &message.title,
        error: MessageModel::validate_title(&message.title).as_deref(),
    })
    .render()
    {
        Ok(form_title) => {
            (StatusCode::OK, csrf, Html(form_title)).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[instrument(level = "debug", skip(csrf))]
pub async fn validate_content(
    Path(id): Path<i32>,
    csrf: CsrfToken,
    Extension(token): Extension<Arc<String>>,
    Form(message): Form<MessageModel>,
) -> impl IntoResponse {
    match (MessageFormContentTemplate {
        token: &token,
        id,
        value: &message.content,
        error: MessageModel::validate_content(&message.content).as_deref(),
    })
    .render()
    {
        Ok(form_content) => {
            (StatusCode::OK, csrf, Html(form_content)).into_response()
        }
        Err(error) => {
            error!("{error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
