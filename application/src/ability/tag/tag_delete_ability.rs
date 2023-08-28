use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use common::errorx::Errorx;
use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use super::cmd::tag_delete_cmd::{TagDeleteCmd, TAG_DEL_CHILD, TAG_DEL_INHERENT, TAG_DEL_ROOT};

pub struct TagDeleteAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    pub tag_repository: TR,
    pub ctx: Arc<AppContext>,
    tag: Option<TagAggregate>,
}

#[async_trait::async_trait]
impl<TR> IAbility for TagDeleteAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    type R = TagAggregate;
    type CMD = TagDeleteCmd;

    // 检测名字、父标签是否已存在
    async fn check_handler(&mut self, cmd: &Self::CMD) -> anyhow::Result<()> {
        let tag = match __self.tag_repository.by_id(cmd.id).await {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    anyhow::bail!(Errorx::new(
                        self.ctx.locale,
                        common::i18n::I18nKey::TagNotExist
                    ))
                }
            },
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TagQuery
                ))
            }
        };

        match cmd.del_type {
            TAG_DEL_CHILD => {}
            TAG_DEL_INHERENT => {}
            TAG_DEL_ROOT => {}
            _ => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::ParamError
                ))
            }
        };

        self.tag = Some(tag);

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
