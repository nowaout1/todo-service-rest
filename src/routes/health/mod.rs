use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub fn create_health_router() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> impl IntoResponse {
    tracing::info!("Got request to `ping`");

    (StatusCode::OK, "pong")
}
