use middleware::{auth::UsersRepository, session::SessionPool};
use model::repository::FoodsRepository;

pub mod error_type;
pub mod handlers;
pub mod middleware;
pub mod model;
pub mod routers;

#[derive(Debug, Clone)]
pub struct AppState {
    pub foods_repo: FoodsRepository,
    pub users_repo: UsersRepository,
    pub session_store: SessionPool,
}

impl AppState {
    pub fn new(
        foods_repo: FoodsRepository,
        users_repo: UsersRepository,
        session_store: SessionPool,
    ) -> Self {
        Self {
            foods_repo,
            users_repo,
            session_store,
        }
    }
}
