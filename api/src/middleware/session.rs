use crate::AppState;

use super::auth::User;
use axum::{async_trait, extract::FromRef};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Error)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("unexpected error")]
    Unexpected,
}

#[derive(Debug, Clone)]
pub struct SessionPool {
    pub pool: PgPool,
}

impl SessionPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait SessionManage<'a, T, S>: Clone + Send + Sync + 'static
where
    T: Clone + Send + Sync + Serialize + Deserialize<'a>,
    S: Clone + PartialEq,
{
    type UserInfo;
    type SessionId;
    type Error;

    async fn create_session(&self, target_user: T) -> Result<Self::SessionId, Self::Error>;
    async fn verify_session(&self, session_id: S) -> Result<Self::UserInfo, Self::Error>;
}

#[async_trait]
impl SessionManage<'_, User, String> for SessionPool {
    type UserInfo = User;
    type SessionId = String;
    type Error = SessionError;

    async fn create_session(&self, target_user: User) -> Result<Self::SessionId, Self::Error> {
        let session_id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
INSERT INTO session 
(session_id, user_info) VALUES ($1, $2)
RETURNING *
        "#,
        )
        .bind(&session_id)
        .bind(target_user.id)
        .bind(target_user.user_name)
        .execute(&self.pool)
        .await
        .map_err(|_e| SessionError::Unexpected)?;

        Ok(session_id)
    }

    async fn verify_session(&self, session_id: String) -> Result<Self::UserInfo, Self::Error> {
        let user_info = sqlx::query_as::<_, User>(
            r#"
SELECT * FROM session 
WHERE session_id = $1
        "#,
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_e| SessionError::Unexpected)?;

        user_info.ok_or(SessionError::NotFound)
    }
}

impl FromRef<AppState> for SessionPool {
    fn from_ref(input: &AppState) -> Self {
        input.session_store.clone()
    }
}