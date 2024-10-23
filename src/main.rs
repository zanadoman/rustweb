mod controllers;
mod models;
mod routes;
mod templates;

use std::sync::Arc;

use axum::serve;
use sqlx::MySqlPool;
use tokio::{main, net::TcpListener};
use tracing_subscriber::{fmt, fmt::format::FmtSpan};

use routes::message_routes::message_routes;

#[main]
async fn main() {
    fmt()
        .with_span_events(FmtSpan::FULL)
        .with_target(false)
        .init();
    serve(
        TcpListener::bind("127.0.0.1:8000").await.unwrap(),
        message_routes().with_state(Arc::new(
            MySqlPool::connect("mariadb://root:12345678@localhost/messages")
                .await
                .unwrap(),
        )),
    )
    .await
    .unwrap();
}
