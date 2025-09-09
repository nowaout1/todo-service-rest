use tokio::net::TcpListener;

use axum::Router;

use todo_service_rest::{
    config::Config,
    routes::{health::create_health_router, todo::create_todo_router},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(error) = dotenvy::dotenv() {
        tracing::error!("failed to initialize .env file: {error:?}");
    }

    tracing::info!("Running server...");

    let cfg = Config::parse();
    let listener = TcpListener::bind(&cfg.addr).await?;
    let router = Router::new()
        .merge(create_health_router())
        .merge(create_todo_router());

    axum::serve(listener, router).await?;

    Ok(())
}
