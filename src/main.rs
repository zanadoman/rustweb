mod controllers;
mod models;
mod routes;
mod services;
mod templates;

use std::{env::var, error::Error, sync::Arc};

use axum::{extract::Request, middleware::from_fn, serve};
use axum_csrf::{CsrfConfig, CsrfLayer};
use dotenv::dotenv;
use routes::routes;
use services::{
    authenticator::AuthenticatorService,
    csrf::{csrf_provider, csrf_verifier},
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
    let listener = TcpListener::bind(var("APP_ADDRESS")?.as_str()).await?;
    info!("{listener:?}");
    let state = StateService::new(var("DATABASE_URL")?.as_str()).await?;
    info!("{state:?}");
    serve(
        listener,
        routes()
            .layer(from_fn(csrf_verifier))
            .layer(from_fn(csrf_provider))
            .layer(CsrfLayer::new(CsrfConfig::default()))
            .layer(AuthenticatorService::new(state.database.clone()).await?)
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
            .with_state(Arc::new(state))
            .nest_service("/assets", ServeDir::new("./assets")),
    )
    .with_graceful_shutdown(async { ctrl_c().await.unwrap() })
    .await?;
    Ok(())
}
