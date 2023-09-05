use base::ddd::command::ICommand;
use domain::aggregate::preclude::*;

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct TagCreateCmd {
    #[validate(length(min = 1, max = 10))]
    pub name: String,
    #[validate(range(min = 0))]
    pub parent_id: i32,
    #[validate(range(min = 0))]
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
            next: Vec::new(),
            prev: vec![self.parent_id],
        }
    }
}
