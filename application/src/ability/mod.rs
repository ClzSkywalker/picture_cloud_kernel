pub mod share;
pub mod tag;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::preclude::*;
use infrastructure::db::repository::new_tag_repository;
use std::sync::Arc;

use self::tag::{cmd::TagCreateCmd, tag_create_ability::TagCreateAbility};

pub fn new_tag_create_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = TagAggregate, CMD = TagCreateCmd> {
    TagCreateAbility {
        tag_repository: new_tag_repository(ctx.clone()),
        ctx: ctx.clone(),
    }
}
