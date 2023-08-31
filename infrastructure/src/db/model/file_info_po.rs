use super::preclude::*;
use domain::share::value_object::file_prop::FilePorp;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "file_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub name: String,
    pub created_time: DateTimeLocal,
    pub file_type: String,
    pub width: i32,
    pub height: i32,
    pub score: i32, // 0~100
    pub size: i32,  // kb
    pub prop: FilePorp,
    pub classify_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Classify,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Classify => Entity::belongs_to(ClassifyEntity)
                .from(Column::ClassifyId)
                .to(ClassifyColumn::Id)
                .into(),
        }
    }
}

impl Related<ClassifyEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Classify.def()
    }
}

impl Related<TagInfoEntity> for Entity {
    fn to() -> RelationDef {
        TagToFileRelation::TagInfo.def()
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
            created_time: Set(self.created_time),
            file_type: Set(self.file_type.clone()),
            width: Set(self.width),
            height: Set(self.height),
            score: Set(self.score),
            size: Set(self.size),
            prop: Set(self.prop.clone()),
            classify_id: Set(self.classify_id),
        }
    }
}
