use crate::{
    error_type::{RepositoryError, ServerError},
    model::{
        repository::{CreateFood, FoodsRepository, UpdateFood},
        CrudForDb,
    },
};
use axum::{
    async_trait,
    extract::{FromRequest, Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
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

pub async fn post_food(
    State(repository): State<FoodsRepository>,
    ValidatedJson(payload): ValidatedJson<CreateFood>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = repository.create(payload).await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_food(
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read(id).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn get_all_foods(
    State(repository): State<FoodsRepository>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read_all().await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn update_food(
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFood>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.update(id, payload).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn delete_food(
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::NO_CONTENT)
}
