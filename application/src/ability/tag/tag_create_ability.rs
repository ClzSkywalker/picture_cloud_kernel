use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use super::cmd::TagCreateCmd;

pub struct TagCreateAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    pub tag_repository: TR,
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl<TR> IAbility for TagCreateAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    type R = TagAggregate;
    type CMD = TagCreateCmd;

    async fn check_handler(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let tag = cmd.to_ag();

        let tag = match self.tag_repository.insert(tag).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e);
            }
        };
        Ok(tag)
    }
}
