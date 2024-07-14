use axum::{async_trait, extract::FromRef};
use password_auth::{generate_hash, verify_password, VerifyError};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as, PgPool};
use thiserror::Error;
use tokio::task;
use validator::Validate;

use crate::AppState;

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

// データベースに保存する型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    mail: String,
    password: String,
}

// 新規ユーザ作成
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1))]
    user_name: String,
    #[validate(email)]
    mail: String,
    #[validate(length(min = 8))]
    password: String,
}

impl CreateUser {
    pub fn new(user_name: String, mail: String, password: String) -> Self {
        Self {
            user_name,
            mail,
            password,
        }
    }
}

// ログイン認証のデータ
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credential {
    pub mail: String,
    pub password: String,
}

impl Credential {
    pub fn new(mail: String, password: String) -> Self {
        Self {
            mail,
            password,
        }
    }
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

#[async_trait]
impl Auth<CreateUser, Credential, '_> for UsersRepository {
    type Response = User;
    type Error = AuthError;

    async fn create_user(&self, payload: CreateUser) -> Result<Self::Response, Self::Error> {
        // ユーザーが既に登録されているかをメールアドレスを用いて確認する
        let isexist = query_as::<_, User>(
            r#"
SELECT * FROM users
WHERE mail = $1
            "#,
        )
        .bind(&payload.mail)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_e| AuthError::Unexpected)?;

        // ユーザがなかったら、登録処理をする
        match isexist {
            None => {
                let hashed_password = task::spawn_blocking(|| generate_hash(payload.password))
                    .await
                    .map_err(|_e| AuthError::Unexpected)?;

                let user = query_as::<_, User>(
                    r#"
        INSERT INTO users
        (user_name, mail, password) VALUES ($1, $2, $3)
        RETURNING *
                "#,
                )
                .bind(payload.user_name)
                .bind(payload.mail.clone())
                .bind(hashed_password)
                .fetch_one(&self.pool)
                .await
                .map_err(|_e| AuthError::Unexpected)?;

                Ok(user)
            }
            Some(_user) => Err(AuthError::AlredyExists),
        }
    }

    async fn verify_user(&self, credential: Credential) -> Result<Self::Response, Self::Error> {
        let stored_user = query_as::<_, User>(
            r#"
SELECT * FROM users 
WHERE mail = $1
        "#,
        )
        .bind(&credential.mail)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| AuthError::UserNotFound)?;

        let cloned_user = stored_user.clone();
        let verify_result = task::spawn_blocking(move || {
            verify_password(&credential.password, &cloned_user.password)
        })
        .await
        .map_err(|_e| AuthError::Unexpected)?;

        match verify_result {
            Ok(_) => Ok(stored_user),
            Err(e) => match e {
                VerifyError::Parse(_) => Err(AuthError::Unexpected),
                VerifyError::PasswordInvalid => Err(AuthError::Defferentpassword),
            },
        }
    }
}

impl FromRef<AppState> for UsersRepository {
    fn from_ref(input: &AppState) -> Self {
        input.users_repo.clone()
    }
}
