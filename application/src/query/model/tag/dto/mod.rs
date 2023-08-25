use serde::Serialize;

use infrastructure::db::model::preclude::*;

#[derive(Debug, Serialize)]
pub struct TagInfoItem {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub sort: i32,
}

impl From<TagInfoModel> for TagInfoItem {
    fn from(value: TagInfoModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            parent_id: value.parent_id,
            sort: value.sort,
        }
    }
}
