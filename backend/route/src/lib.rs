use axum::http::StatusCode;
use axum::{routing::get, Router};
pub fn route() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .fallback(fallback)
}

pub async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
