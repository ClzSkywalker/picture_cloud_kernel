use crate::aggregate::preclude::*;
use base::ddd::repository::IRepository;

#[async_trait::async_trait]
pub trait ITagRespository: IRepository<AG = TagAggregate, ID = i32> {
    async fn find_by_name(&self, name: String) -> anyhow::Result<Option<TagAggregate>>;
}
