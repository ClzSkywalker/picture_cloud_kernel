pub mod share;
pub mod tag;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::preclude::*;
use infrastructure::db::repository::new_tag_repository;
use std::sync::Arc;

use self::tag::{
    cmd::{tag_create_cmd::TagCreateCmd, tag_update_cmd::TagUpdateCmd},
    tag_create_ability::TagCreateAbility,
    tag_update_ability::TagUpdateAbility,
};

pub fn new_tag_create_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = TagAggregate, CMD = TagCreateCmd> {
    TagCreateAbility {
        tag_repository: new_tag_repository(ctx.clone()),
        ctx: ctx.clone(),
    }
}

pub fn new_tag_update_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = TagAggregate, CMD = TagUpdateCmd> {
    TagUpdateAbility {
        tag_repository: new_tag_repository(ctx.clone()),
        ctx: ctx.clone(),
    }
}
