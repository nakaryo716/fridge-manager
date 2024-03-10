use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use crate::{
    error_type::RepositoryError,
    store::{
        app_logic::RepositoryForDb,
        data_type::{CreateItem, UpdateItem, ValidatedJson},
    },
};

pub async fn index() -> impl IntoResponse {
    Html("<h1>Test</h1>")
}

// 各ハンドラは自動的にHandler Traitを実装できるようにしなければならない
// 各ハンドラの第一引数は，Sate(): Clone + Send + 'static を実装
// 第二引数に'staticを実装しているものにする

// postハンドラ
pub async fn post_item<T: RepositoryForDb>(
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateItem>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = repository.create(payload).await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::CREATED, Json(response)))
}

// getハンドラ
pub async fn get_item<T: RepositoryForDb>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read(id).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

// all-getハンドラ
pub async fn get_all_item<T: RepositoryForDb>(
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read_all().await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

// updateハンドラ
pub async fn update_item<T: RepositoryForDb>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItem>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.update(id, payload).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

// deleteハンドラ
pub async fn delete_item<T: RepositoryForDb>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::NO_CONTENT)
}
