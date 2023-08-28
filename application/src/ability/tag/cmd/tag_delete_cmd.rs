use base::ddd::command::ICommand;
use domain::aggregate::preclude::*;

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct TagDeleteCmd {
    pub id: i32,
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
