use base::ddd::command::ICommand;
use domain::aggregate::preclude::*;

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct TagUpdateCmd {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub sort: i32,
}

impl ICommand for TagUpdateCmd {}

impl TagUpdateCmd {
    pub fn to_ag(&self) -> TagAggregate {
        TagAggregate {
            id: self.id,
            name: self.name.clone(),
            parent_id: self.parent_id,
            sort: self.sort,
            next: None,
            prev: None,
        }
    }
}
