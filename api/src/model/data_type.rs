use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};

// データベースpool構造体
#[derive(Debug, Clone)]
pub struct ItemRepository {
    pg_pool: PgPool,
}

impl ItemRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}

// 賞味(消費)期限を定義する構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpirationDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

// もしかしていらないかも
// databaseにinsertする際に使用(?)
impl ExpirationDate {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
        }
    }
}

// データベースに使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub expiration_date: ExpirationDate,
    pub used: bool,
}

// app_logic::create()に使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub expiration_date: ExpirationDate,
}

// app_logic::update()に使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub expiration_date: Option<ExpirationDate>,
    pub used: Option<bool>,
}

// ValidatedJsonのtrait実装
