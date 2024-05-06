use crate::{
    handlers::repository_handles::{delete_food, get_all_foods, get_food, post_food, update_food},
    model::repository::CrudForDb,
};
use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// ルーティングの設定
// 食品の追加 -> method: 'POST' uri: '/fridge'
// 任意IDの食品のクエリ -> method: 'GET' uri: '/fridge:id'
// すべての食品のクエリ -> method: 'GET' uri: '/fridge'
// 任意IDの食品の編集 -> method: 'PUT' uri: '/fridge:id'
// 任意ID食品の削除 -> method: 'DELETE' uri: '/fridge:id'
pub fn services<'a, S, T, R>(repository: S) -> Router
where
    S: CrudForDb<'a, T, R>,
    T: Serialize + Deserialize<'a> + FromRow<'a, R> + 'static,
    R: Row,
{
    Router::new()
        .route("/fridge", post(post_food).get(get_all_foods))
        .route(
            "/fridge/:id",
            get(get_food).put(update_food).delete(delete_food),
        )
        .with_state(Arc::new(repository))
        .layer(
            CorsLayer::new()
                // "http://localhost:5173".parse::<HeaderValue>().unwrap()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE])
                .allow_headers(vec![CONTENT_TYPE]),
        )
}
