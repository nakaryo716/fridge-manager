use axum::response::{Html, IntoResponse};

pub async fn index() -> impl IntoResponse {
    tracing::info!("Called index handler");
    Html("<h1>Test</h1>")
}

// postハンドラ

// getハンドラ

// all-getハンドラ

// findハンドラ

// deleteハンドラ

// defaultハンドラ
// Page NotFoundを表示