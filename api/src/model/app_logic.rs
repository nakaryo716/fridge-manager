use super::data_type::{CreateItem, Item, ItemRepository, UpdateItem};
use crate::error_type::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait RepositoryForDb: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateItem) -> Result<Item, RepositoryError>;
    async fn read(&self, id: i32) -> Result<Item, RepositoryError>;
    async fn read_all(&self) -> Result<Vec<Item>, RepositoryError>;
    async fn update(&self, id: i32, payload: UpdateItem) -> Result<Item, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

#[async_trait]
impl RepositoryForDb for ItemRepository {
    // postの内容をdatabaseにinsert
    async fn create(&self, payload: CreateItem) -> Result<Item, RepositoryError> {
        let item = sqlx::query_as::<_, Item>(r#"
INSERT INTO item (name, expiration_date)
VALUES ($1, $2)
RETURNING *
        "#)
        .bind(payload.name)
        .bind(payload.expiration_date)
        .fetch_one(&self.pg_pool)
        .await
       .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(0),
            _ => RepositoryError::Unexpected,     
       })?;
        Ok(item)
    }

    // 任意のidのデータをdatabaseから取得
    async fn read(&self, id: i32) -> Result<Item, RepositoryError> {
        let item = sqlx::query_as::<_, Item>(r#"
SELECT * FROM item
WHERE id = $1
        "#)
        .bind(id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    // データベースにあるすべてのデータの取得
    async fn read_all(&self) -> Result<Vec<Item>, RepositoryError> {
        let item = sqlx::query_as::<_, Item>(r#"
SELECT * FROM item
        "#)
        .fetch_all(&self.pg_pool)
        .await
        .map_err(|e| match  e  {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(0),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    // 任意のidのデータを更新
    async fn update(&self, id: i32, payload: UpdateItem) -> Result<Item, RepositoryError> {
        Ok(todo!())
    }

    // 任意のidのデータを削除
    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        Ok(todo!())
    }
}
