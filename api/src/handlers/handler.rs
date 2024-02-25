use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use crate::model::{app_logic::RepositoryForDb, data_type::CreateItem};

pub async fn index() -> impl IntoResponse {
    tracing::info!("Called index handler");
    Html("<h1>Test</h1>")
}

// postハンドラ
pub async fn post_item<T: RepositoryForDb>(
    Json(payload): Json<CreateItem>,
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = repository.create(payload).await.unwrap();

    Ok((StatusCode::CREATED, Json(response)))
}

// getハンドラ

// all-getハンドラ

// findハンドラ

// deleteハンドラ

// defaultハンドラ
// Page NotFoundを表示
