use super::RepositoryForDb;
use crate::error_type::RepositoryError;
use crate::store::{CreateItem, Item, ItemRepository, UpdateItem};
use async_trait::async_trait;

#[async_trait]
impl RepositoryForDb for ItemRepository {
    // postの内容をdatabaseにinsert
    async fn create(&self, payload: CreateItem) -> Result<Item, RepositoryError> {
        let item = sqlx::query_as::<_, Item>(
            r#"
INSERT INTO item (name, expiration_date)
VALUES ($1, $2)
RETURNING *
        "#,
        )
        .bind(payload.name)
        .bind(payload.expiration_date)
        .fetch_one(&self.pg_pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    // 任意のidのデータをdatabaseから取得
    async fn read(&self, id: i32) -> Result<Item, RepositoryError> {
        let item = sqlx::query_as::<_, Item>(
            r#"
SELECT * FROM item
WHERE id = $1
        "#,
        )
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
        let item = sqlx::query_as::<_, Item>(
            r#"
SELECT * FROM item
ORDER BY id
        "#,
        )
        .fetch_all(&self.pg_pool)
        .await
        .or(Err(RepositoryError::Unexpected))?;

        Ok(item)
    }

    // 任意のidのデータを更新
    async fn update(&self, id: i32, payload: UpdateItem) -> Result<Item, RepositoryError> {
        let old_item = self.read(id).await?;

        let insert_name = match payload.name {
            Some(value) => value,
            None => old_item.name,
        };

        let insert_expiration_date = match payload.expiration_date {
            Some(value) => value,
            None => old_item.expiration_date,
        };

        let insert_used = match payload.used {
            Some(value) => value,
            None => old_item.used,
        };

        let item = sqlx::query_as::<_, Item>(
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
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(item)
    }

    // 任意のidのデータを削除
    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
DELETE FROM item
WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFoud(id),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(())
    }
}
