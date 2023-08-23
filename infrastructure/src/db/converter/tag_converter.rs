use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(t: TagInfoModel) -> TagAggregate {
    TagAggregate {
        id: t.id,
        name: t.name,
        prev: None,
        next: None,
    }
}
