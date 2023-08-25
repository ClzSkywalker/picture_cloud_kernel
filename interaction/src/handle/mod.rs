use axum::{routing::get, routing::post, Router};

use self::tag::{tag_create, tag_find};

pub mod tag;

pub fn unauth_api() -> Router {
    Router::new().nest("/tag", tag_api())
}

pub fn tag_api() -> Router {
    Router::new()
        .route("/", post(tag_create))
        .route("/", get(tag_find))
}
