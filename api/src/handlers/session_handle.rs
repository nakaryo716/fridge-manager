use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::middleware::session_extract::SessionData;

// Front側がセッションを持っているかどうかを確認するためのハンドラー
// なければ、SessionData Extractorの方でエラーが返されている
pub async fn is_session(
    SessionData(user_info): SessionData,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, Json(user_info)))
}
