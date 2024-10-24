mod controllers;
mod models;
mod routes;
mod services;
mod templates;

use std::{env::var, error::Error};

use axum::{extract::Request, serve};
use dotenv::dotenv;
use services::authenticator::AuthenticatorService;
use sqlx::MySqlPool;
use tokio::{main, net::TcpListener, signal::ctrl_c};
use tower_http::trace::TraceLayer;
use tracing::{info, span, Level};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    registry,
    util::SubscriberInitExt,
    EnvFilter,
};

use routes::routes;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    registry()
        .with(EnvFilter::try_from_default_env()?)
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .init();
    let listener = TcpListener::bind(var("APP_ADDRESS")?.as_str()).await?;
    info!("{:?}", listener);
    let database = MySqlPool::connect(var("DATABASE_URL")?.as_str()).await?;
    info!("{:?}", database);
    serve(
        listener,
        routes()
            .layer(TraceLayer::new_for_http().make_span_with(
                |request: &Request| {
                    span! {
                        Level::DEBUG,
                        "request",
                        method = ?request.method(),
                        route = ?request.uri(),
                    }
                },
            ))
            .layer(AuthenticatorService::new(&database).await?)
            .with_state(database.into()),
    )
    .with_graceful_shutdown(async { ctrl_c().await.unwrap() })
    .await?;
    Ok(())
}
