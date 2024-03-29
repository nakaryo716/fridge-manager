use crate::{
    handlers::{delete_item, get_all_item, get_item, post_item, test, update_item},
    store::RepositoryForDb,
};
use axum::{
    routing::{get, post},
    Router,
};
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
}
