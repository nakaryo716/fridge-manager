use crate::{
    handlers::handler::{delete_item, get_all_item, get_item, index, post_item, update_item},
    store::app_logic::RepositoryForDb,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

// ルーティングの設定
pub fn app<T: RepositoryForDb>(repository: T) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/fridge", post(post_item).get(get_all_item))
        .route(
            "/fridge/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(Arc::new(repository))
}
