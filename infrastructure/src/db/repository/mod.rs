use common::contextx::AppContext;
use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use self::tag_repository::TagRepository;
use std::sync::Arc;

pub mod tag_repository;

pub fn new_tag_repository<'a>(
    ctx: &'a AppContext,
) -> impl ITagRespository<AG = TagAggregate, ID = i32> {
    TagRepository { ctx: ctx.clone() }
}
