use crate::{
    error_type::{RepositoryError, ServerError},
    model::repository::{CreateFood, CrudForDb, UpdateFood},
};
use axum::{
    async_trait,
    extract::{FromRequest, Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::sync::Arc;
use validator::Validate;

// バリエーションされたJsonをExtracterとして扱うための
// ラッパー構造体及びFromRequestトレイトの実装
#[derive(Debug, Clone)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

// トレイト境界がいろいろ書かれているが、単純にデータベースとのやり取りができる
//Crudトレイトが実装されたリポジトリをStateとして置いておくことを示しているだけ
pub async fn post_food<'a, S, T, R>(
    State(repository): State<Arc<S>>,
    ValidatedJson(payload): ValidatedJson<CreateFood>,
) -> Result<impl IntoResponse, StatusCode>
where
    S: CrudForDb<'a, T, R>,
    T: Serialize + Deserialize<'a> + FromRow<'a, R>,
    R: Row,
{
    let response = repository.create(payload).await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_food<'a, S, T, R>(
    State(repository): State<Arc<S>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode>
where
    S: CrudForDb<'a, T, R>,
    T: Serialize + Deserialize<'a> + FromRow<'a, R>,
    R: Row,
{
    let item = repository.read(id).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn get_all_foods<'a, S, T, R>(
    State(repository): State<Arc<S>>,
) -> Result<impl IntoResponse, StatusCode>
where
    S: CrudForDb<'a, T, R>,
    T: Serialize + Deserialize<'a> + FromRow<'a, R>,
    R: Row,
{
    let item = repository.read_all().await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn update_food<'a, S, T, R>(
    State(repository): State<Arc<S>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFood>,
) -> Result<impl IntoResponse, StatusCode>
where
    S: CrudForDb<'a, T, R>,
    T: Serialize + Deserialize<'a> + FromRow<'a, R>,
    R: Row,
{
    let item = repository.update(id, payload).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn delete_food<'a, S, T, R>(
    State(repository): State<Arc<S>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode>
where
    S: CrudForDb<'a, T, R>,
    T: Serialize + Deserialize<'a> + FromRow<'a, R>,
    R: Row,
{
    repository.delete(id).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::NO_CONTENT)
}
