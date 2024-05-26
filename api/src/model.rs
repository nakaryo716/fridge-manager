use crate::error_type::RepositoryError;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use validator::Validate;

pub mod auth;
pub mod repository;

// DBに対して一般的なCRUD操作を実装させるトレイト
// 戻り値は任意の種類(Pg, MySql, SqLite)から得たデータが
// Json化することができることをトレイト境界として指定している
#[async_trait]
pub trait CrudForDb<'a, S, T, U>:
    Clone + std::marker::Send + std::marker::Sync + 'static
where
    S: Row,
    T: Deserialize<'a> + Validate + Clone,
    U: Deserialize<'a> + Validate + Clone,
{
    type Response: Serialize + Deserialize<'a> + FromRow<'a, S>;

    async fn create(&self, payload: T) -> Result<Self::Response, RepositoryError>;
    async fn read(&self, id: i32) -> Result<Self::Response, RepositoryError>;
    async fn read_all(&self) -> Result<Vec<Self::Response>, RepositoryError>;
    async fn update(&self, id: i32, payload: U) -> Result<Self::Response, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
