mod controllers;
mod models;
mod routes;
mod services;
mod templates;

use std::env::var;

use axum::serve;
use dotenv::dotenv;
use services::authenticator::AuthenticatorService;
use sqlx::MySqlPool;
use tokio::{main, net::TcpListener};
use tracing_subscriber::{fmt, fmt::format::FmtSpan};

use routes::routes;

#[main]
async fn main() {
    dotenv().expect(".env missing");

    fmt()
        .with_span_events(FmtSpan::FULL)
        .with_target(false)
        .init();

    let database = MySqlPool::connect(var("DATABASE_URL").unwrap().as_str())
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
