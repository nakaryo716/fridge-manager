use axum::{routing::get, Router};

use crate::handlers::handler::index;

pub fn app() -> Router {
    Router::new().route("/", get(index))
}
