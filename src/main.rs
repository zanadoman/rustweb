mod controllers;
mod models;
mod routes;
mod services;
mod templates;

use axum::serve;
use services::authenticator::AuthenticatorService;
use sqlx::MySqlPool;
use tokio::{main, net::TcpListener};
use tracing_subscriber::{fmt, fmt::format::FmtSpan};

use routes::routes;

#[main]
async fn main() {
    fmt()
        .with_span_events(FmtSpan::FULL)
        .with_target(false)
        .init();

    let database =
        MySqlPool::connect("mariadb://root:12345678@localhost/messages")
            .await
            .unwrap();

    serve(
        TcpListener::bind("0.0.0.0:8000").await.unwrap(),
        routes()
            .layer(AuthenticatorService::new(&database).await.unwrap())
            .with_state(database.into()),
    )
    .await
    .unwrap();
}
