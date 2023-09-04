use common::contextx::SharedStateCtx;

use self::{
    repository::tag_cqrs_repository::TagCqrsRepository, service::tag_cqrs_service::TagCqrsService,
};

pub mod assembler;
pub mod model;
pub mod repository;
pub mod service;

pub fn new_tag_cqrs_service(ctx: SharedStateCtx) -> TagCqrsService {
    TagCqrsService {
        ctx: ctx.clone(),
        tag_cqrs_respository: TagCqrsRepository { ctx: ctx.clone() },
    }
}
