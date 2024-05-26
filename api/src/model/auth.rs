use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserData {
    user_id: i32,
    user_name: String,
    credential: Credential,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credential {
    mail: String,
    password: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    user_name: String,
    credential: Credential,
}



// ユーザー作成
// 1. 名前とメール、パスワードの設定
// 2. 同じメールがないか確認
// 3. データベースに追加
