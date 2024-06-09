use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;

use crate::middleware::session::{SessionError, SessionManage, SessionPool};

use super::auth_handle::SESSION_ID;

pub async fn is_session(
    jar: CookieJar,
    State(session_store): State<SessionPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let value = jar.get(SESSION_ID).map(|cookie| cookie.value().to_owned());

    match value {
        Some(cookie_value) => {
            let user = session_store
                .verify_session(&cookie_value)
                .await
                .map_err(|e| match e {
                    SessionError::NotFound => StatusCode::NOT_FOUND,
                    SessionError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                })?;
            Ok((StatusCode::OK, Json(user)))
        }
        None => Err(StatusCode::NON_AUTHORITATIVE_INFORMATION),
    }
}
