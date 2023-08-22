use anyhow::Result;
use async_trait::async_trait;

use super::event::{BaseDomainEvent, IDomainType};

/// 领域事件仓储，保存已发生的领域事件，用于事件溯源
#[async_trait]
pub trait IDomainEventRepository {
    type Data;
    type E: IDomainType;
    async fn load(id: String) -> Result<BaseDomainEvent<Self::Data, Self::E>>;
    async fn load_by_domain_id(
        domain_id: String,
    ) -> Result<Vec<BaseDomainEvent<Self::Data, Self::E>>>;
    async fn save(event: BaseDomainEvent<Self::Data, Self::E>) -> Result<()>;
    async fn update(event: BaseDomainEvent<Self::Data, Self::E>) -> Result<()>;
}
