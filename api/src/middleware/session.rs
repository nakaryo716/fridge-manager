use super::auth::User;
use crate::AppState;
use axum::{async_trait, extract::FromRef};
use axum_session_manager::SessionManage;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Error)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("unexpected error")]
    Unexpected,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SessionInfo {
    pub user_id: i32,
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
impl SessionManage<User> for SessionPool {
    type SessionID = String;
    type UserInfo = SessionInfo;
    type Error = SessionError;

    async fn add_session(&self, session_data: User) -> Result<Self::SessionID, Self::Error> {
        let session_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
INSERT INTO session 
(session_id, user_id) VALUES ($1, $2)
        "#,
        )
        .bind(&session_id)
        .bind(&session_data.user_id)
        .execute(&self.pool)
        .await
        .map_err(|_e| SessionError::Unexpected)?;

        Ok(session_id)
    }

    async fn verify_session(
        &self,
        session_id: &str,
    ) -> Result<Option<Self::UserInfo>, Self::Error> {
        let session_info = sqlx::query_as::<_, SessionInfo>(
            r#"
SELECT user_id FROM session 
WHERE session_id = $1
            "#,
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_e| SessionError::Unexpected)?;

        Ok(session_info)
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), Self::Error> {
        sqlx::query(
            r#"
DELETE FROM session
WHERE session_id = $1
            "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .map_err(|_e| SessionError::NotFound)?;

        Ok(())
    }
}

impl FromRef<AppState> for SessionPool {
    fn from_ref(input: &AppState) -> Self {
        input.session_store.clone()
    }
}
