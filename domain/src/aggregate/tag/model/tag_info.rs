use base::ddd::aggregate::IAggregate;

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub next: Option<Vec<i32>>,
    pub prev: Option<Vec<i32>>,
}

impl IAggregate for TagInfo {}
