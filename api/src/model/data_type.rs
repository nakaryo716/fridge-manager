use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, PgPool};
use sqlx::Row;
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

// 賞味(消費)期限を定義する構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpirationDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

// データベースに使用する構造体の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub expiration_date: ExpirationDate,
    pub used: bool,
}

// .query_as()を使うためにItemにFromRow Traitを手動実装
impl FromRow<'_, PgRow> for Item {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            expiration_date: ExpirationDate {
                year: row.try_get("year")?,
                month: row.try_get("month")?,
                day: row.try_get("day")?,
            },
            used: row.try_get("used")?,
        })
    }
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
