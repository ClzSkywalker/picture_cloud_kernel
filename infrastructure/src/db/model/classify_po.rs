use super::preclude::*;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "classify")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub name: String,
    pub parent_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    FileInfo,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::FileInfo => Entity::has_many(FileInfoEntity).into(),
        }
    }
}

impl Related<FileInfoEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FileInfo.def()
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
            parent_id: Set(self.parent_id),
        }
    }
}
