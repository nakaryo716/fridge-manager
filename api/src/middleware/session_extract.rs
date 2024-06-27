use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::CookieJar;

use super::{
    auth::User,
    session::{SessionError, SessionInfo, SessionManage, SessionPool},
};
use crate::handlers::auth_handle::SESSION_ID;

// すべてのエンドポイント(ハンドラー)でセッションを監視するために
// extractorとして実装して、middlewareとして扱う
// ハンドラー部ではSessionDataを受け取ることができ、ユーザー情報(ユーザーid,ユーザー名)を知ることができる
// セッションがない(サインインされていない)場合はこのSessionDataのFromRequestPartsトレイトの実装部で
//　弾かれるようになっている

#[derive(Debug, Clone)]
pub struct SessionData(pub SessionInfo);

#[async_trait]
impl<'a, S> FromRequestParts<S> for SessionData
where
    CookieJar: FromRequestParts<S>,
    SessionPool:
        FromRef<S> + SessionManage<'a, User, String, UserInfo = SessionInfo, Error = SessionError>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session_pool = SessionPool::from_ref(state);
        // リクエストからcookieを取得
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
        let cookie_value = jar.get(SESSION_ID).map(|cookie| cookie.value().to_owned());

        match cookie_value {
            Some(cookie_value) => {
                let user =
                    session_pool
                        .verify_session(&cookie_value)
                        .await
                        .map_err(|e| match e {
                            SessionError::NotFound => StatusCode::UNAUTHORIZED,
                            SessionError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                        })?;
                Ok(SessionData(user))
            }
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
