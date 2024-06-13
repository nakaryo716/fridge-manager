use crate::{
    error_type::{RepositoryError, ServerError},
    middleware::session_extract::SessionData,
    model::repository::{CreateFood, CrudForDb, FoodsRepository, UpdateFood},
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
    SessionData(user_info): SessionData,
    State(repository): State<FoodsRepository>,
    ValidatedJson(payload): ValidatedJson<CreateFood>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = repository.create(payload, user_info).await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_food(
    SessionData(user_info): SessionData,
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read(id, user_info).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn get_all_foods(
    SessionData(user_info): SessionData,
    State(repository): State<FoodsRepository>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.read_all(user_info).await.map_err(|e| match e {
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn update_food(
    SessionData(user_info): SessionData,
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFood>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = repository.update(id, payload, user_info).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn delete_food(
    SessionData(user_info): SessionData,
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id, user_info).await.map_err(|e| match e {
        RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::NO_CONTENT)
}
