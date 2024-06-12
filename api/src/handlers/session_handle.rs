use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::middleware::session_extract::SessionData;

// SessionDataエキストラクターのテストのためのハンドラー
pub async fn is_session(
    SessionData(user_info): SessionData,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, Json(user_info)))
}
