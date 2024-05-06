use api::{routers, model::ItemRepository};
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
    let db_connection_str = std::env::var("DATABASE_URL").expect("can't find database");
    println!("{}", db_connection_str);

    // database 接続
    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let respository = ItemRepository::new(pool);

    // ルーティング
    let app = routers::app(respository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::info!("Listening on {:?}", listener);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
