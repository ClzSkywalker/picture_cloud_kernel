use base::ddd::ability::IAbility;
use base::ddd::repository::IRepository;
use common::contextx::SharedStateCtx;
use common::errorx::Errorx;
use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use super::cmd::tag_create_cmd::TagCreateCmd;

pub struct TagCreateAbility<TR>
where
    TR: ITagRespository + IRepository<AG = TagAggregate, ID = i32>,
{
    pub ctx: SharedStateCtx,
    pub tag_repository: TR,
}

#[async_trait::async_trait]
impl<TR> IAbility for TagCreateAbility<TR>
where
    TR: ITagRespository + IRepository<AG = TagAggregate, ID = i32>,
{
    type R = TagAggregate;
    type CMD = TagCreateCmd;

    // 检测名字、父标签是否已存在
    async fn check_handler(&mut self, cmd: &Self::CMD) -> anyhow::Result<()> {
        let ctx = self.ctx.lock().await;
        match __self.tag_repository.exist_name(cmd.name.clone()).await {
            Ok(r) => {
                if r {
                    anyhow::bail!(Errorx::new(ctx.locale, common::i18n::I18nKey::TagNameExist))
                }
            }
            Err(_) => {
                anyhow::bail!(Errorx::new(ctx.locale, common::i18n::I18nKey::TagQuery))
            }
        };

        if cmd.parent_id == 0 {
            return Ok(());
        }

        match __self.tag_repository.exist_parent_id(cmd.parent_id).await {
            Ok(r) => {
                if !r {
                    anyhow::bail!(Errorx::new(
                        ctx.locale,
                        common::i18n::I18nKey::TagParentNotExist
                    ))
                }
            }
            Err(_) => {
                anyhow::bail!(Errorx::new(ctx.locale, common::i18n::I18nKey::TagQuery))
            }
        };

        Ok(())
    }
    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&mut self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
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
