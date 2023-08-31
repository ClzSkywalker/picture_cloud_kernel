use super::preclude::*;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "tag_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub name: String,
    pub sort: i32,
    pub parent_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl Related<FileInfoEntity> for Entity {
    fn to() -> RelationDef {
        TagToFileRelation::FileInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_active_base(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            created_at: NotSet,
            updated_at: NotSet,
            deleted_at: NotSet,
            name: Set(self.name.clone()),
            sort: Set(self.sort),
            parent_id: Set(self.parent_id),
        }
    }

    pub fn table_name() -> String {
        String::from("tag_info")
    }
}
