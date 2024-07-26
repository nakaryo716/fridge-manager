use crate::{
    error_type::{RepositoryError, ServerError},
    middleware::session::SessionInfo,
    model::repository::{CreateFood, CrudForDb, FoodsRepository, UpdateFood},
};
use axum::{
    async_trait,
    extract::{FromRequest, Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use axum_session_manager::{UserData, UserState};
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

// Extensionでユーザーのセッション状態を確認している。
pub async fn post_food(
    Extension(user_data): Extension<UserData<SessionInfo>>,
    State(repository): State<FoodsRepository>,
    ValidatedJson(payload): ValidatedJson<CreateFood>,
) -> Result<impl IntoResponse, StatusCode> {
    match user_data.0 {
        UserState::HaveSession(session_data) => {
            let response = repository
                .create(payload, session_data)
                .await
                .map_err(|e| match e {
                    RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                    _ => StatusCode::SERVICE_UNAVAILABLE,
                })?;

            Ok((StatusCode::CREATED, Json(response)))
        }
        UserState::NoSession => Err(StatusCode::UNAUTHORIZED),
        UserState::NoCookie => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn get_food(
    Extension(user_data): Extension<UserData<SessionInfo>>,
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    match user_data.0 {
        UserState::HaveSession(session_data) => {
            let item = repository
                .read(id, session_data)
                .await
                .map_err(|e| match e {
                    RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
                    RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                })?;

            Ok((StatusCode::OK, Json(item)))
        }
        UserState::NoSession => Err(StatusCode::UNAUTHORIZED),
        UserState::NoCookie => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn get_all_foods(
    Extension(user_data): Extension<UserData<SessionInfo>>,
    State(repository): State<FoodsRepository>,
) -> Result<impl IntoResponse, StatusCode> {
    match user_data.0 {
        UserState::HaveSession(session_data) => {
            let item = repository
                .read_all(session_data)
                .await
                .map_err(|e| match e {
                    RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                    _ => StatusCode::SERVICE_UNAVAILABLE,
                })?;

            Ok((StatusCode::OK, Json(item)))
        }
        UserState::NoSession => Err(StatusCode::UNAUTHORIZED),
        UserState::NoCookie => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn update_food(
    Extension(user_data): Extension<UserData<SessionInfo>>,
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFood>,
) -> Result<impl IntoResponse, StatusCode> {
    match user_data.0 {
        UserState::HaveSession(session_data) => {
            let item = repository
                .update(id, payload, session_data)
                .await
                .map_err(|e| match e {
                    RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
                    RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                })?;

            Ok((StatusCode::OK, Json(item)))
        }
        UserState::NoSession => Err(StatusCode::UNAUTHORIZED),
        UserState::NoCookie => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn delete_food(
    Extension(user_data): Extension<UserData<SessionInfo>>,
    State(repository): State<FoodsRepository>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    match user_data.0 {
        UserState::HaveSession(session_data) => {
            repository
                .delete(id, session_data)
                .await
                .map_err(|e| match e {
                    RepositoryError::NotFoud(_) => StatusCode::NOT_FOUND,
                    RepositoryError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                })?;

            Ok(StatusCode::NO_CONTENT)
        }
        UserState::NoSession => Err(StatusCode::UNAUTHORIZED),
        UserState::NoCookie => Err(StatusCode::UNAUTHORIZED),
    }
}
