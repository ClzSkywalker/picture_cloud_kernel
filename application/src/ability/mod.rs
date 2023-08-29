pub mod share;
pub mod tag;

use self::tag::{
    cmd::{
        tag_create_cmd::TagCreateCmd, tag_delete_cmd::TagDeleteCmd, tag_update_cmd::TagUpdateCmd,
    },
    tag_create_ability::TagCreateAbility,
    tag_delete_ability::TagDeleteAbility,
    tag_update_ability::TagUpdateAbility,
};
use base::ddd::ability::IAbility;
use domain::aggregate::preclude::*;
use infrastructure::db::repository::new_tag_repository;

pub fn new_tag_create_ability() -> impl IAbility<R = TagAggregate, CMD = TagCreateCmd> {
    TagCreateAbility {
        tag_repository: new_tag_repository(),
    }
}

pub fn new_tag_delete_ability() -> impl IAbility<R = TagAggregate, CMD = TagDeleteCmd> {
    TagDeleteAbility {
        tag_repository: new_tag_repository(),
        tag: None,
    }
}

pub fn new_tag_update_ability() -> impl IAbility<R = TagAggregate, CMD = TagUpdateCmd> {
    TagUpdateAbility {
        tag_repository: new_tag_repository(),
    }
}
