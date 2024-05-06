use crate::error_type::{RepositoryError, ServerError};
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{types::chrono, FromRow, PgPool};
use validator::Validate;
pub mod app_logic;

#[async_trait]
pub trait RepositoryForDb: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateItem) -> Result<Item, RepositoryError>;
    async fn read(&self, id: i32) -> Result<Item, RepositoryError>;
    async fn read_all(&self) -> Result<Vec<Item>, RepositoryError>;
    async fn update(&self, id: i32, payload: UpdateItem) -> Result<Item, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

// データベースpool構造体
#[derive(Debug, Clone)]
pub struct ItemRepository {
    pub pg_pool: PgPool,
}

impl ItemRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}

// データベースに使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub expiration_date: chrono::NaiveDate,
    pub used: bool,
}

// app_logic::create()に使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateItem {
    #[validate(length(min = 1, max = 100, message = "validated error was occurred"))]
    pub name: String,
    pub expiration_date: chrono::NaiveDate,
}

// app_logic::update()に使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateItem {
    #[validate(length(min = 1, max = 100, message = "validated error was occurred"))]
    pub name: Option<String>,
    pub expiration_date: Option<chrono::NaiveDate>,
    pub used: Option<bool>,
}

// ValidatedJsonをRequestにするためのFromRequest trait実装
#[derive(Debug, Clone)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}
