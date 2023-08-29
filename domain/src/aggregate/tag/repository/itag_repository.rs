use crate::aggregate::preclude::*;
use base::ddd::repository::IRepository;
use common::contextx::AppContext;

#[async_trait::async_trait]
pub trait ITagRespository: IRepository<AG = TagAggregate, ID = i32> {
    async fn del_by_ids(&self, ctx: &mut AppContext, ids: Vec<i32>) -> anyhow::Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-29
    /// Description    : 更新ids中的父id
    /// param           {*} self
    /// param           {Vec} ids
    /// param           {i32} parent_id
    /// return          {*}
    ///    
    async fn update_parent_by_ids(
        &self,
        ctx: &mut AppContext,
        ids: Vec<i32>,
        parent_id: i32,
    ) -> anyhow::Result<()>;

    async fn find_by_name(
        &self,
        ctx: &mut AppContext,
        name: String,
    ) -> anyhow::Result<Option<TagAggregate>>;
    async fn exist_name(&self, ctx: &mut AppContext, name: String) -> anyhow::Result<bool>;
    async fn exist_parent_id(&self, ctx: &mut AppContext, id: i32) -> anyhow::Result<bool>;
}
