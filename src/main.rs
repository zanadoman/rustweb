mod controllers;
mod models;
mod routes;
mod services;
mod templates;

use std::{env::var, error::Error, sync::Arc};

use axum::{extract::Request, middleware::from_fn, serve};
use axum_csrf::{CsrfConfig, CsrfLayer};
use dotenvy::dotenv;
use routes::routes;
use services::{
    authenticator::AuthenticatorService, integrity::integrity_service,
    state::StateService,
};
use tokio::{main, net::TcpListener, signal::ctrl_c};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, span, Level};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    registry,
    util::SubscriberInitExt,
    EnvFilter,
};

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    registry()
        .with(EnvFilter::try_from_default_env()?)
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .init();
    let listener = TcpListener::bind(&var("APP_ADDRESS")?).await?;
    info!("{listener:?}");
    let state = Arc::new(StateService::new(&var("DATABASE_URL")?).await?);
    info!("{state:?}");
    serve(
        listener,
        routes()
            .layer(AuthenticatorService::new(state.database().clone()).await?)
            .layer(from_fn(integrity_service))
            .layer(CsrfLayer::new(CsrfConfig::default()))
            .layer(TraceLayer::new_for_http().make_span_with(
                |request: &Request| {
                    span! {
                        Level::INFO,
                        "request",
                        method = %request.method(),
                        route = %request.uri(),
                    }
                },
            ))
            .with_state(state)
            .nest_service("/assets", ServeDir::new("./assets")),
    )
    .with_graceful_shutdown(async { ctrl_c().await.unwrap() })
    .await?;
    Ok(())
}
