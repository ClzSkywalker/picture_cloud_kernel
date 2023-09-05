use base::ddd::aggregate::IAggregate;

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub id: i32,
    pub name: String,
    pub sort: i32,
    pub parent_id: i32,
    pub next: Vec<i32>,
    pub prev: Vec<i32>,
}

impl TagInfo {
    // 判断结构是否为空
    pub fn is_not_empty(&self) -> bool {
        if self.id <= 0 {
            return false;
        }
        if self.name.eq("") {
            return false;
        }
        if self.sort == 0 {
            return false;
        }
        return true;
    }
}

impl IAggregate for TagInfo {}
