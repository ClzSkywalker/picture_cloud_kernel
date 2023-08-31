use super::preclude::*;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "tag_to_file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub tag_id: i32,
    pub file_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    FileInfo,
    TagInfo,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::FileInfo => Entity::belongs_to(FileInfoEntity)
                .from(Column::FileId)
                .to(FileInfoColumn::Id)
                .into(),
            Relation::TagInfo => Entity::belongs_to(TagInfoEntity)
                .from(Column::TagId)
                .to(TagInfoColumn::Id)
                .into(),
        }
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
            file_id: Set(self.file_id),
            tag_id: Set(self.tag_id),
        }
    }
}
