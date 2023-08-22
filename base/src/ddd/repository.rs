use super::aggregate::IAggregate;

#[async_trait::async_trait]
pub trait IRepository: Sync + Send {
    //
    type AG: IAggregate;
    // 唯一标识
    type ID;
    async fn insert(&self, ag: Self::AG) -> anyhow::Result<Self::AG>;
    async fn delete(&self, id: Self::ID) -> anyhow::Result<()>;
    async fn update(&self, ag: Self::AG) -> anyhow::Result<()>;
    async fn by_id(&self, id: Self::ID) -> anyhow::Result<Option<Self::AG>>;
}
