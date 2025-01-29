use axum::{routing::get, Router};

pub struct Api;

impl Api {
    pub fn build() -> Router {
        Router::new().route("/", get(|| async { "Hello, World!" }))
    }
}
