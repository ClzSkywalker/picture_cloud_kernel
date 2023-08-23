use base::ddd::value_object::IValueObject;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, FromJsonQueryResult)]
pub struct FilePorp {
    pub colors: Vec<String>,
}

impl IValueObject for FilePorp {}
