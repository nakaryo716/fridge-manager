use axum::{routing::get, Router};

use crate::handlers::handler::index;

// ルーティングの設定
// get, post, put, deleteに対応する各ハンドラのマッピング
// detabaseの全体共有".state(Arc::new())"
// CORSの設定".layer()"
// default設定(Page NotFoud)
pub fn app() -> Router {
    Router::new().route("/", get(index))
}