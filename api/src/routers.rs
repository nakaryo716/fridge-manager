use std::marker::PhantomData;

use crate::{
    handlers::{
        auth_handle::{sign_in, sign_out, sign_up, SESSION_ID},
        repository_handles::{delete_food, get_all_foods, get_food, post_food, update_food},
        session_handle::is_session,
    },
    AppState,
};
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    Router,
};

use axum_session_manager::SessionManagerLayer;
use tower_http::cors::CorsLayer;

// ルーティングの設定
// 食品の追加 -> method: 'POST' uri: '/fridge'
// 任意IDの食品のクエリ -> method: 'GET' uri: '/fridge:id'
// すべての食品のクエリ -> method: 'GET' uri: '/fridge'
// 任意IDの食品の編集 -> method: 'PUT' uri: '/fridge:id'
// 任意ID食品の削除 -> method: 'DELETE' uri: '/fridge:id'
pub fn services(state: AppState) -> Router {
    let layer = SessionManagerLayer::new(
        state.session_store.clone(),
        SESSION_ID,
        PhantomData::default(),
    );

    Router::new()
        .route("/fridge", post(post_food).get(get_all_foods))
        .route(
            "/fridge/:id",
            get(get_food).put(update_food).delete(delete_food),
        )
        .route("/sign_up", post(sign_up))
        .route("/sign_in", post(sign_in))
        .route("/is_session", get(is_session))
        .route("/sign_out", get(sign_out))
        .layer(layer)
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE])
                .allow_headers(vec![CONTENT_TYPE])
                .allow_credentials(true),
        )
}

#[cfg(test)]
mod test {
    use std::usize;

    use crate::{
        middleware::{
            auth::{CreateUser, Credential, UsersRepository},
            session::{SessionInfo, SessionPool},
        },
        model::repository::FoodsRepository,
        routers::services,
        AppState,
    };
    use axum::{body::Body, http::request};
    use http::StatusCode;
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    const USERNAME: &str = "test_user1";
    const USERMAIL: &str = "testmail2@gmail.com";
    const USERPASS: &str = "testpassword1234";

    #[tokio::test]
    async fn signup_test() {
        let app_state = database_connection().await;
        let app = services(app_state);

        let body = CreateUser::new(
            USERNAME.to_string(),
            USERMAIL.to_string(),
            USERPASS.to_string(),
        );

        let req = request::Builder::new()
            .uri("http://localhost:3000/sign_up")
            .method("POST")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(serde_json::to_string(&body).unwrap())
            .unwrap();

        let res = app.oneshot(req).await.unwrap().status();
        assert_eq!(StatusCode::OK, res);
    }

    #[tokio::test]
    async fn signin_test() {
        let app_state = database_connection().await;
        let app = services(app_state);

        let body = Credential::new(USERMAIL.to_string(), USERPASS.to_string());

        let req = request::Builder::new()
            .uri("http://localhost:3000/sign_in")
            .method("POST")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body(serde_json::to_string(&body).unwrap())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn is_session_test() {
        let app_state = database_connection().await;
        let app = services(app_state);

        let body = Credential::new(USERMAIL.to_string(), USERPASS.to_string());

        let req = request::Builder::new()
            .uri("http://localhost:3000/sign_in")
            .method("POST")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body(serde_json::to_string(&body).unwrap())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(StatusCode::OK, res.status());

        let cookies = res
            .headers()
            .get(http::header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap();

        let req2 = request::Builder::new()
            .uri("http://localhost:3000/is_session")
            .method("GET")
            .header(http::header::COOKIE, cookies)
            .header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req2).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let body = String::from_utf8(body).unwrap();
        let json: SessionInfo = serde_json::from_str(&body).unwrap();
        println!("{:?}", json)
    }

    // database接続ヘルパー関数
    async fn database_connection() -> AppState {
        dotenvy::dotenv().unwrap();
        let db_url = std::env::var("DATABASE_URL").expect("can't find database");

        // database 接続
        let pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .expect("can't connect to database");

        let foods_repo = FoodsRepository::new(pool.clone());
        let users_repo = UsersRepository::new(pool.clone());
        let session_store = SessionPool::new(pool.clone());
        AppState::new(foods_repo, users_repo, session_store)
    }
}
