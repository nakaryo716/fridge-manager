use crate::error_type::RepositoryError;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::chrono, FromRow, PgPool};
use std::fmt::Debug;
use validator::Validate;

use super::CrudForDb;

// 冷蔵庫内の食品のパラメータ
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Food {
    pub food_id: i32,
    pub food_name: String,
    pub expiration: chrono::NaiveDate,
    pub used: bool,
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

// CrudForDbトレイトの実装部分
// Postgresからの操作によるJsonシリアライズ/デシリアライズ可能
// Food構造体を戻り値として実装
#[async_trait]
impl CrudForDb<'_, Food, PgRow, CreateFood, UpdateFood> for FoodsRepository {
    async fn create(&self, payload: CreateFood) -> Result<Food, RepositoryError> {
        let item = sqlx::query_as::<_, Food>(
            r#"
INSERT INTO item (food_name, expiration)
VALUES ($1, $2)
RETURNING *
        "#,
        )
        .bind(payload.food_name)
        .bind(payload.expiration)
        .fetch_one(&self.pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    async fn read(&self, id: i32) -> Result<Food, RepositoryError> {
        let item = sqlx::query_as::<_, Food>(
            r#"
SELECT * FROM item
WHERE food_id = $1
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    async fn read_all(&self) -> Result<Vec<Food>, RepositoryError> {
        let item = sqlx::query_as::<_, Food>(
            r#"
SELECT * FROM item
ORDER BY food_id
        "#,
        )
        .fetch_all(&self.pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    async fn update(&self, id: i32, payload: UpdateFood) -> Result<Food, RepositoryError> {
        let old_item = self.read(id).await?;

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
WHERE id = $4
RETURNING *
        "#,
        )
        .bind(insert_food)
        .bind(insert_expiration)
        .bind(insert_used)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
DELETE FROM item
WHERE food_id = $1
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(())
    }
}
