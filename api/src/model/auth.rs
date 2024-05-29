use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as, PgPool};
use thiserror::Error;

// データベースに保存する型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    mail: String,
    password: String,
}

// 新規ユーザ作成
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    user_name: String,
    mail: String,
    password: String,
}

// ログイン認証のデータ
#[derive(Debug, Clone, Deserialize)]
pub struct Credential {
    pub mail: String,
    pub password: String,
}

// ユーザーデータを保持するデータベース
#[derive(Debug, Clone)]
pub struct UsersRepository {
    pool: PgPool,
}

impl UsersRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
pub trait Auth<NewUser, Credential, 'a>: Clone + Send + Sync + 'static
where
    NewUser: Clone + Deserialize<'a>,
    Credential: Clone + Deserialize<'a>,
{
    type Response;
    type Error;

    async fn create_user(&self, payload: NewUser) -> Result<Self::Response, Self::Error>;
    async fn verify_user(&self, credential: Credential) -> Result<Self::Response, Self::Error>;
}

#[derive(Debug, Clone, Error)]
pub enum AuthError {
    #[error("User alredy exists")]
    AlredyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Wrong password")]
    Defferentpassword,
    #[error("Unexpected Error")]
    Unexpected,
}

#[async_trait]
impl Auth<CreateUser, Credential, '_> for UsersRepository {
    type Response = User;
    type Error = AuthError;

    async fn create_user(&self, payload: CreateUser) -> Result<Self::Response, Self::Error> {
        // 要検証⚠️
        // このクエリは失敗する恐れあり
        // sqlxがこのサブクエリでどのようにデータをバインドするかわからない
        let user = query_as::<_, User>(
            r#"
INSERT INTO user 
(user_name, mail, password) VALUES ($1, $2, $3) 
IF NOT EXISTS (SELECT mail FROM user WHERE mail = $1)
RETURNING *
        "#,
        )
        .bind(payload.user_name)
        .bind(payload.mail)
        .bind(payload.password)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| AuthError::AlredyExists)?;

        Ok(user)
    }

    async fn verify_user(&self, credential: Credential) -> Result<Self::Response, Self::Error> {
        let stored_user = query_as::<_, User>(
            r#"
SELECT * FROM user 
WHERE mail = $1
        "#,
        )
        .bind(&credential.mail)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| AuthError::UserNotFound)?;

        if credential.password == stored_user.password {
            Ok(stored_user)
        } else {
            Err(AuthError::Defferentpassword)
        }
    }
}

// ユーザー作成
// 1. 名前とメール、パスワードの設定
// 2. 同じメールがないか確認
// 3. データベースに追加
