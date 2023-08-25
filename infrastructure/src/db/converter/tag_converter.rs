use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(
    t: TagInfoModel,
    prevs: Option<Vec<TagInfoModel>>,
    nexts: Option<Vec<TagInfoModel>>,
) -> TagAggregate {
    let prevs: Option<Vec<i32>> = match prevs {
        Some(r) => Some(r.iter().map(|item| item.id).collect()),
        None => None,
    };
    let nexts: Option<Vec<i32>> = match nexts {
        Some(r) => Some(r.iter().map(|item| item.id).collect()),
        None => None,
    };
    TagAggregate {
        id: t.id,
        name: t.name,
        sort: t.sort,
        parent_id: t.parent_id,
        prev: prevs,
        next: nexts,
    }
}

pub fn serialize(u: TagAggregate) -> TagInfoModel {
    TagInfoModel {
        created_at: None,
        updated_at: None,
        deleted_at: None,
        id: u.id,
        name: u.name,
        sort: u.sort,
        parent_id: u.parent_id,
    }
}
