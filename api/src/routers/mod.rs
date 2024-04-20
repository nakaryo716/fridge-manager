use crate::{
    handlers::{delete_item, get_all_item, get_item, post_item, test, update_item},
    store::RepositoryForDb,
};
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method}, routing::{get, post}, Router
};
use tower_http::cors::CorsLayer;
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
                .allow_origin("http://100.64.1.18:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE])
                .allow_headers(vec![CONTENT_TYPE,])
        )
}
