use base::ddd::command::ICommand;
use domain::aggregate::preclude::*;

use serde::Deserialize;
use validator::Validate;

pub mod tag_create_cmd;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct TagCreateCmd {
    pub name: String,
    pub parent_id: i32,
    pub sort: i32,
}

impl ICommand for TagCreateCmd {}

impl TagCreateCmd {
    pub fn to_ag(&self) -> TagAggregate {
        TagAggregate {
            id: 0,
            name: self.name.clone(),
            parent_id: self.parent_id,
            sort: self.sort,
            next: None,
            prev: None,
        }
    }
}
