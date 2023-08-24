use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(
    t: TagInfoModel,
    prevs: Vec<TagInfoModel>,
    nexts: Vec<TagInfoModel>,
) -> TagAggregate {
    let prevs: Vec<i32> = prevs.iter().map(|item| item.id).collect();
    let nexts: Vec<i32> = nexts.iter().map(|item| item.id).collect();
    TagAggregate {
        id: t.id,
        name: t.name,
        parent_id: t.parent_id,
        prev: Some(prevs),
        next: Some(nexts),
    }
}

pub fn serialize(u: TagAggregate) -> TagInfoModel {
    TagInfoModel {
        created_at: None,
        updated_at: None,
        deleted_at: None,
        id: u.id,
        name: u.name,
        parent_id: u.parent_id,
    }
}
