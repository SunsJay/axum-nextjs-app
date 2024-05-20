
use axum::{routing::get, Router};

pub fn route() -> Router {
      Router::new().route("/", get(|| async { "Hello, World!" }))
}
