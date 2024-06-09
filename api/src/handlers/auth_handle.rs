use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::middleware::{
    auth::{Auth, AuthError, CreateUser, Credential, UsersRepository},
    session::{SessionManage, SessionPool},
};

pub const SESSION_ID: &str = "session_id";

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
    jar: CookieJar,
    State(user_repository): State<UsersRepository>,
    State(session_store): State<SessionPool>,
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

    // session_idをcookieとしてを渡す
    let session_id_value = session_store
        .create_session(&res)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = jar.add(Cookie::new(SESSION_ID, session_id_value));
    Ok((StatusCode::OK, cookie, Json(res)))
}
