use api::routers::marge_route;
use tracing::Level;

#[tokio::main]
async fn main() {
    // ログの設定
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    tracing::info!("Starting application");

    // ルーティング
    let app = marge_route::app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("Listening on {:?}", listener);

    axum::serve(listener, app).await.unwrap();
}
