use crate::{error_type::RepositoryError, middleware::session::SessionInfo, AppState};
use axum::{async_trait, extract::FromRef};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::chrono, FromRow, PgPool, Row};
use std::{error::Error, fmt::Debug};
use validator::Validate;

// 冷蔵庫内の食品のパラメータ
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Food {
    pub food_id: i32,
    pub food_name: String,
    pub expiration: chrono::NaiveDate,
    pub used: bool,
    pub user_id: i32,
}

// クライアント側で管理する食品を追加する為の構造体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFood {
    #[validate(length(min = 1, max = 100, message = "validated error was occurred"))]
    pub food_name: String,
    pub expiration: chrono::NaiveDate,
}

// クライアント側で編集する食品を追加する為の構造体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFood {
    #[validate(length(min = 1, max = 100, message = "validated error was occurred"))]
    pub food_name: Option<String>,
    pub expiration: Option<chrono::NaiveDate>,
    pub used: Option<bool>,
}

// Food構造体を管理するためのラッパー構造体
// 本実装ではPostgresを採用
#[derive(Debug, Clone)]
pub struct FoodsRepository {
    pub pool: PgPool,
}

impl FoodsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// DBに対して一般的なCRUD操作を実装させるトレイト
// 戻り値は任意の種類(Pg, MySql, SqLite)から得たデータが
// Json化することができることをトレイト境界として指定している
#[async_trait]
pub trait CrudForDb<'a, R, N, U, I>:
    Clone + std::marker::Send + std::marker::Sync + 'static
where
    R: Row,
    N: Deserialize<'a> + Validate + Clone,
    U: Deserialize<'a> + Validate + Clone,
    I: Clone + Send + Sync,
{
    type Response: Serialize + Deserialize<'a> + FromRow<'a, R>;
    type Error: Debug + Error;

    async fn create(&self, payload: N, user_info: I) -> Result<Self::Response, Self::Error>;
    async fn read(&self, id: i32, user_info: I) -> Result<Self::Response, Self::Error>;
    async fn read_all(&self, user_info: I) -> Result<Vec<Self::Response>, Self::Error>;
    async fn update(
        &self,
        id: i32,
        payload: U,
        user_info: I,
    ) -> Result<Self::Response, Self::Error>;
    async fn delete(&self, id: i32, user_info: I) -> Result<(), Self::Error>;
}

// CrudForDbトレイトの実装部分
// Postgresからの操作によるJsonシリアライズ/デシリアライズ可能
// Food構造体を戻り値として実装
#[async_trait]
impl CrudForDb<'_, PgRow, CreateFood, UpdateFood, SessionInfo> for FoodsRepository {
    type Response = Food;
    type Error = RepositoryError;

    async fn create(
        &self,
        payload: CreateFood,
        user_info: SessionInfo,
    ) -> Result<Self::Response, Self::Error> {
        let item = sqlx::query_as::<_, Food>(
            r#"
INSERT INTO item (food_name, expiration, user_id)
VALUES ($1, $2, $3)
RETURNING *
        "#,
        )
        .bind(payload.food_name)
        .bind(payload.expiration)
        .bind(user_info.user_id)
        .fetch_one(&self.pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    async fn read(&self, id: i32, user_info: SessionInfo) -> Result<Self::Response, Self::Error> {
        let item = sqlx::query_as::<_, Food>(
            r#"
SELECT * FROM item
WHERE food_id = $1 AND user_id = $2
        "#,
        )
        .bind(id)
        .bind(user_info.user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    async fn read_all(&self, user_info: SessionInfo) -> Result<Vec<Self::Response>, Self::Error> {
        let item = sqlx::query_as::<_, Food>(
            r#"
SELECT * FROM item
WHERE user_id = $1
ORDER BY expiration
        "#,
        )
        .bind(user_info.user_id)
        .fetch_all(&self.pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    async fn update(
        &self,
        id: i32,
        payload: UpdateFood,
        user_info: SessionInfo,
    ) -> Result<Self::Response, Self::Error> {
        let old_item = self.read(id, user_info.clone()).await?;

        let insert_food = match payload.food_name {
            Some(value) => value,
            None => old_item.food_name,
        };

        let insert_expiration = match payload.expiration {
            Some(value) => value,
            None => old_item.expiration,
        };

        let insert_used = match payload.used {
            Some(value) => value,
            None => old_item.used,
        };

        let item = sqlx::query_as::<_, Food>(
            r#"
UPDATE item SET (food_name, expiration, used) = ($1, $2, $3)
WHERE food_id = $4 AND user_id = $5
RETURNING *
        "#,
        )
        .bind(insert_food)
        .bind(insert_expiration)
        .bind(insert_used)
        .bind(id)
        .bind(user_info.user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    async fn delete(&self, id: i32, user_info: SessionInfo) -> Result<(), Self::Error> {
        sqlx::query(
            r#"
DELETE FROM item
WHERE food_id = $1 AND user_id = $2
        "#,
        )
        .bind(id)
        .bind(user_info.user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(())
    }
}

impl FromRef<AppState> for FoodsRepository {
    fn from_ref(input: &AppState) -> Self {
        input.foods_repo.clone()
    }
}
