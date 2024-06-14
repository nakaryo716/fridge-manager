use api::{
    middleware::{auth::UsersRepository, session::SessionPool},
    model::repository::FoodsRepository,
    routers,
};
use sqlx::postgres::PgPoolOptions;
use std::error;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // ログの設定
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    tracing::info!("Starting application");

    // database_urlの設定
    dotenvy::dotenv()?;
    let db_url = std::env::var("DATABASE_URL").expect("can't find database");

    // database 接続
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("can't connect to database");

    let foods_repo = FoodsRepository::new(pool.clone());
    let users_repo = UsersRepository::new(pool.clone());
    let session_store = SessionPool::new(pool.clone());

    let app_state = api::AppState::new(foods_repo, users_repo, session_store);

    // ルーティング
    let services = routers::services(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {:?}", listener);

    axum::serve(listener, services).await.unwrap();

    Ok(())
}
