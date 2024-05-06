use crate::error_type::RepositoryError;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::chrono, FromRow, PgPool, Row};
use validator::Validate;

// 冷蔵庫内の食品のパラメータ
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub expiration: chrono::NaiveDate,
    pub used: bool,
}

// クライアント側で管理する食品を追加する為の構造体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFood {
    #[validate(length(min = 1, max = 100, message = "validated error was occurred"))]
    pub name: String,
    pub expiration: chrono::NaiveDate,
}

// クライアント側で編集する食品を追加する為の構造体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFood {
    #[validate(length(min = 1, max = 100, message = "validated error was occurred"))]
    pub name: Option<String>,
    pub expiration_date: Option<chrono::NaiveDate>,
    pub used: Option<bool>,
}

// DBに対して一般的なCRUD操作を実装させるトレイト
// 戻り値は任意の種類(Pg, MySql, SqLite)から得たデータが
// Json化することができることをトレイト境界として指定している
#[async_trait]
pub trait CrudForDb<'a, T, R>: Clone + std::marker::Send + std::marker::Sync + 'static
where
    T: Serialize + Deserialize<'a> + FromRow<'a, R>,
    R: Row,
{
    async fn create(&self, payload: CreateFood) -> Result<T, RepositoryError>;
    async fn read(&self, id: i32) -> Result<T, RepositoryError>;
    async fn read_all(&self) -> Result<Vec<T>, RepositoryError>;
    async fn update(&self, id: i32, payload: UpdateFood) -> Result<T, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
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

// Postgresからの操作によるJsonシリアライズ/デシリアライズ可能
// Food構造体を戻り値として実装
#[async_trait]
impl CrudForDb<'_, Food, PgRow> for FoodsRepository {
    async fn create(&self, payload: CreateFood) -> Result<Food, RepositoryError> {
        let item = sqlx::query_as::<_, Food>(
            r#"
INSERT INTO item (name, expiration_date)
VALUES ($1, $2)
RETURNING *
        "#,
        )
        .bind(payload.name)
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
WHERE id = $1
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
ORDER BY id
        "#,
        )
        .fetch_all(&self.pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    async fn update(&self, id: i32, payload: UpdateFood) -> Result<Food, RepositoryError> {
        let old_item = self.read(id).await?;

        let insert_name = match payload.name {
            Some(value) => value,
            None => old_item.name,
        };

        let insert_expiration_date = match payload.expiration_date {
            Some(value) => value,
            None => old_item.expiration,
        };

        let insert_used = match payload.used {
            Some(value) => value,
            None => old_item.used,
        };

        let item = sqlx::query_as::<_, Food>(
            r#"
UPDATE item SET (name, expiration_date, used) = ($1, $2, $3)
WHERE id = $4
RETURNING *
        "#,
        )
        .bind(insert_name)
        .bind(insert_expiration_date)
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
WHERE id = $1
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
