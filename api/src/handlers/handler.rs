use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use crate::model::{app_logic::RepositoryForDb, data_type::CreateItem};

pub async fn index() -> impl IntoResponse {
    tracing::info!("Called index handler");
    Html("<h1>Test</h1>")
}
// .unwrap()はあとで修正
// エラーハンドリングを行う(StatusCodeを返す)

// postハンドラ
pub async fn post_item<T: RepositoryForDb>(
    Json(payload): Json<CreateItem>,
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = repository.create(payload).await.unwrap();

    Ok((StatusCode::CREATED, Json(response)))
}

// getハンドラ
pub async fn get_item<T: RepositoryForDb>(
    Path(id): Path<i32>,
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read(id).await.unwrap();

    Ok((StatusCode::OK, Json(item)))
}

// all-getハンドラ
pub async fn get_all_item<T: RepositoryForDb>(
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read_all().await.unwrap();

    Ok((StatusCode::OK, Json(item)))
}

// findハンドラ

// deleteハンドラ

// defaultハンドラ
// Page NotFoundを表示
