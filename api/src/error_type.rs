use thiserror::Error;

// エラーの定義
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound id is: {0}")]
    NotFoud(i32),
    #[error("Unexpected Error")]
    Unexpected,
}