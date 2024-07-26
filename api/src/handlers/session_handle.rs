use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use axum_session_manager::{UserData, UserState};

use crate::middleware::session::SessionInfo;

// Front側がセッションを持っているかどうかを確認するためのハンドラー
pub async fn is_session(
    Extension(user_data): Extension<UserData<SessionInfo>>,
) -> Result<impl IntoResponse, StatusCode> {
    match user_data.0 {
        UserState::HaveSession(data) => Ok((StatusCode::OK, Json(data))),
        UserState::NoSession => Err(StatusCode::UNAUTHORIZED),
        UserState::NoCookie => Err(StatusCode::UNAUTHORIZED),
    }
}
