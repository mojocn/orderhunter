use crate::config::Config;
use anyhow::Context;
use axum::{extract::Extension, routing::get, Router};
use sqlx::MySqlPool;
use tower_http::trace::TraceLayer;

use std::sync::Arc;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: MySqlPool,
}

pub async fn serve(config: Config, db: MySqlPool) -> anyhow::Result<()> {
    let addr = &config
        .http_addr
        .as_str()
        .parse()
        .context("Failed to parse HTTP address")?;
    println!("Listening on {}", addr);
    let app = api_router()
        .layer(Extension(ApiContext {
            config: Arc::new(config),
            db,
        }))
        .layer(TraceLayer::new_for_http());
    // We use 8080 as our default HTTP server port, it's pretty easy to remember.
    //
    // Note that any port below 1024 needs superuser privileges to bind on Linux,
    // so 80 isn't usually used as a default for that reason.
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    Router::new()

    // users::router()
    // .merge(profiles::router())
    // .merge(articles::router())
}
