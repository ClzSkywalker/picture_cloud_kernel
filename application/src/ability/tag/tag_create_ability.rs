use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use common::errorx::Errorx;
use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use super::cmd::tag_create_cmd::TagCreateCmd;

pub struct TagCreateAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    pub tag_repository: TR,
}

#[async_trait::async_trait]
impl<TR> IAbility for TagCreateAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    type R = TagAggregate;
    type CMD = TagCreateCmd;

    // 检测名字、父标签是否已存在
    async fn check_handler(&mut self, ctx: &mut AppContext, cmd: &Self::CMD) -> anyhow::Result<()> {
        match __self
            .tag_repository
            .exist_name(ctx, cmd.name.clone())
            .await
        {
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

        match __self
            .tag_repository
            .exist_parent_id(ctx, cmd.parent_id)
            .await
        {
            Ok(r) => {
                if r {
                    anyhow::bail!(Errorx::new(ctx.locale, common::i18n::I18nKey::TagNameExist))
                }
            }
            Err(_) => {
                anyhow::bail!(Errorx::new(ctx.locale, common::i18n::I18nKey::TagQuery))
            }
        };

        Ok(())
    }
    async fn check_idempotent(&mut self, _: &mut AppContext, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&mut self, ctx: &mut AppContext, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let tag = cmd.to_ag();

        let tag = match self.tag_repository.insert(ctx, tag).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e);
            }
        };
        Ok(tag)
    }
}
