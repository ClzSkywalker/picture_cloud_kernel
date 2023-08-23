use crate::aggregate::preclude::*;
use base::ddd::repository::IRepository;

#[async_trait::async_trait]
pub trait ITagRespository: IRepository<AG = TagAggregate, ID = i32> {}
