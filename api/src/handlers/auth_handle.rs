use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::middleware::auth::{Auth, AuthError, CreateUser, Credential, UsersRepository};

pub async fn sign_up(
    State(user_repository): State<UsersRepository>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = user_repository
        .create_user(payload)
        .await
        .map_err(|e| match e {
            AuthError::AlredyExists => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn sign_in(
    State(user_repository): State<UsersRepository>,
    Json(credential): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = user_repository
        .verify_user(credential)
        .await
        .map_err(|e| match e {
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::Defferentpassword => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok((StatusCode::OK, Json(res)))
}
