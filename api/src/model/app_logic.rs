use super::data_type::{CreateItem, Item, ItemRepository};
use crate::error_type::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait RepositoryForDb: std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateItem) -> Result<Item, RepositoryError>;
    async fn read(&self) -> Result<Item, RepositoryError>;
    async fn read_all(&self) -> Result<Item, RepositoryError>;
    async fn update(&self) -> Result<Item, RepositoryError>;
    async fn delete(&self) -> Result<(), RepositoryError>;
}

// create function
// postの内容をdatabaseにinsert

// read function
// 任意のidのデータをdatabaseから取得

// read-all function
// データベースにあるすべてのデータの取得

// update function
// 任意のidのデータを更新

// delete
// 任意のidのデータを削除
#[async_trait]
impl RepositoryForDb for ItemRepository {
    async fn create(&self, payload: CreateItem) -> Result<Item, RepositoryError> {
        Ok((todo!()))
    }
    async fn read(&self) -> Result<Item, RepositoryError> {
        Ok((todo!()))
    }
    async fn read_all(&self) -> Result<Item, RepositoryError> {
        Ok((todo!()))
    }
    async fn update(&self) -> Result<Item, RepositoryError> {
        Ok((todo!()))
    }
    async fn delete(&self) -> Result<(), RepositoryError> {
        Ok((todo!()))
    }
}
