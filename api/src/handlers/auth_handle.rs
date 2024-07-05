use axum::{
    async_trait, extract::{FromRequest, Request, State}, http::StatusCode, response::IntoResponse, Json
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::middleware::{
    auth::{Auth, AuthError, CreateUser, Credential, UsersRepository},
    session::{SessionManage, SessionPool},
};

pub const SESSION_ID: &str = "session_id";

#[derive(Debug, Clone)]
pub struct ValidatedCreateUser<T>(T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedCreateUser<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &'_ S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<T>::from_request(req, &state)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let _ = body.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(ValidatedCreateUser(body))
    }
}

pub async fn sign_up(
    State(user_repository): State<UsersRepository>,
    ValidatedCreateUser(payload): ValidatedCreateUser<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    user_repository
        .create_user(payload)
        .await
        .map_err(|e| match e {
            AuthError::AlredyExists => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::OK)
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
    Ok((StatusCode::OK, cookie))
}

pub async fn sign_out(
    jar: CookieJar,
    State(session_store): State<SessionPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let cookie = jar.get(SESSION_ID).map(|cookie| cookie.value().to_owned());

    match cookie {
        Some(session_id) => {
            session_store
                .delete_session(&session_id.to_string())
                .await
                .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        None => Err(StatusCode::UNAUTHORIZED)?,
    }

    let _ = jar.remove(Cookie::from(SESSION_ID));

    Ok(StatusCode::NO_CONTENT)
}
