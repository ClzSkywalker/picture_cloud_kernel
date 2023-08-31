use axum::{middleware, Router};

use crate::handle::unauth_api;

pub fn create_router() -> Router {
    Router::new().nest("/api/v1", global_router())
}

fn global_router() -> Router {
    Router::new().nest("/unauth", set_unauth_middleware(unauth_api()))
    // .nest("/auth", set_auth_middleware(auth_api()))
}

fn set_unauth_middleware(router: Router) -> Router {
    router.layer(middleware::from_fn(middlewarex::ctx::ctx_fn_mid))
}
