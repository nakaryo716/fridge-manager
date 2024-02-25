use crate::{handlers::handler::index, model::app_logic::RepositoryForDb};
use axum::{routing::get, Router};
use std::sync::Arc;

// ルーティングの設定
// get, post, put, deleteに対応する各ハンドラのマッピング
// detabaseの全体共有".state(Arc::new())"
// CORSの設定".layer()"
// default設定(Page NotFoud)
pub fn app<T: RepositoryForDb>(repository: T) -> Router {
    Router::new()
        .route("/", get(index))
        .with_state(Arc::new(repository))
}
