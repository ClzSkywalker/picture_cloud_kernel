use base::ddd::command::ICommand;
use domain::aggregate::preclude::*;

use serde::Deserialize;
use validator::Validate;

type TagDelType = i32;

pub const TAG_DEL_CHILD: TagDelType = 0; // 删除关联的子tag
pub const TAG_DEL_INHERIT: TagDelType = 1; // 删除后子tag继承其位置
pub const TAG_DEL_ROOT: TagDelType = 2; // 删除后子tag放置在root下

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct TagDeleteCmd {
    pub id: i32,
    pub del_type: TagDelType,
}

impl ICommand for TagDeleteCmd {}

impl TagDeleteCmd {
    pub fn to_ag(&self) -> TagAggregate {
        TagAggregate {
            id: self.id,
            name: String::from(""),
            parent_id: 0,
            sort: 0,
            next: None,
            prev: None,
        }
    }
}
