use axum::http::StatusCode;
use axum::{routing::get, Router};

pub fn route() -> Router {
    let user_routes = Router::new().route("/:id", get(|| async { "User" }));
    let api_routes = Router::new()
        .nest("/users", user_routes);

    Router::new().route("/", get(|| async { "Hello, World!" })).nest("/v1/api", api_routes).fallback(fallback)
}


pub async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
