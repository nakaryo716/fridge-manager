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
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

// データベースに使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub deadline: Date,
}

// app_logic::create()に使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

// app_logic::update()に使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub yread: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

// ValidatedJsonのtrait実装
