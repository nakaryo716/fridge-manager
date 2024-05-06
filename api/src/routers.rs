use crate::{
    handlers::{delete_item, get_all_item, get_item, post_item, test, update_item},
    model::RepositoryForDb,
};
use axum::{
    http::{header::CONTENT_TYPE, Method}, routing::{get, post}, Router
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;

// ルーティングの設定
pub fn app<T: RepositoryForDb>(repository: T) -> Router {
    Router::new()
        .route("/", get(test))
        .route("/fridge", post(post_item).get(get_all_item))
        .route(
            "/fridge/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(Arc::new(repository))
        .layer(
            CorsLayer::new()
                // "http://localhost:5173".parse::<HeaderValue>().unwrap()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE])
                .allow_headers(vec![CONTENT_TYPE])
        )
}
