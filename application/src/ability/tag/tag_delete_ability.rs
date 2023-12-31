use base::ddd::ability::IAbility;
use common::contextx::SharedStateCtx;
use common::errorx::Errorx;
use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use super::cmd::tag_delete_cmd::{TagDeleteCmd, TAG_DEL_CHILD, TAG_DEL_INHERIT, TAG_DEL_ROOT};

pub struct TagDeleteAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    pub ctx: SharedStateCtx,
    pub tag_repository: TR,
    pub tag: Option<TagAggregate>,
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
        let tag = match self.tag_repository.by_id(cmd.id).await {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    anyhow::bail!(Errorx::new(
                        self.ctx.read().await.locale,
                        common::i18n::I18nKey::TagNotExist
                    ))
                }
            },
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.read().await.locale,
                    common::i18n::I18nKey::TagQuery
                ))
            }
        };

        self.tag = Some(tag);

        Ok(())
    }
    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&mut self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let tag = cmd.to_ag();

        match self.ctx.write().await.begin().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("e:{}", e);
                anyhow::bail!(Errorx::new(
                    self.ctx.read().await.locale,
                    common::i18n::I18nKey::TagDelErr
                ))
            }
        };

        match self.tag_repository.delete(tag.id).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.read().await.locale,
                    common::i18n::I18nKey::TagDelErr
                ));
            }
        };

        let tag = self.tag.clone().unwrap();
        let res = match cmd.del_type {
            TAG_DEL_CHILD => self.del_child().await,
            TAG_DEL_INHERIT => self.del_inherit().await,
            TAG_DEL_ROOT => self.del_root().await,
            _ => {
                anyhow::bail!(Errorx::new(
                    self.ctx.read().await.locale,
                    common::i18n::I18nKey::ParamError
                ))
            }
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        match __self.ctx.write().await.commit().await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("e:{}", e);
                anyhow::bail!(Errorx::new(
                    self.ctx.read().await.locale,
                    common::i18n::I18nKey::TagDelErr
                ))
            }
        };

        Ok(tag)
    }
}

impl<TR> TagDeleteAbility<TR>
where
    TR: ITagRespository<AG = TagAggregate, ID = i32>,
{
    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-29
    /// Description    : 删除子标签
    /// param           {*} mut
    /// return          {*}
    ///    
    async fn del_child(&mut self) -> anyhow::Result<()> {
        let ctx = self.ctx.read().await;
        let tag = self.tag.clone().unwrap();
        if tag.next.is_empty() {
            return Ok(());
        }
        match self.tag_repository.del_by_ids(tag.next).await {
            Ok(_) => Ok(()),
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    ctx.locale,
                    common::i18n::I18nKey::TagUpdateChild
                ));
            }
        }
    }

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-29
    /// Description    : 删除后子标签继承父标签位置
    /// param           {*} mut
    /// return          {*}
    ///    
    async fn del_inherit(&mut self) -> anyhow::Result<()> {
        let ctx = self.ctx.read().await;
        let tag = self.tag.clone().unwrap();
        if tag.next.is_empty() {
            return Ok(());
        }
        match self
            .tag_repository
            .update_parent_by_ids(tag.next, tag.parent_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    ctx.locale,
                    common::i18n::I18nKey::TagUpdateChild
                ));
            }
        }
    }

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-29
    /// Description    :  删除后子标签回到root
    /// param           {*} mut
    /// return          {*}
    ///    
    async fn del_root(&mut self) -> anyhow::Result<()> {
        let ctx = self.ctx.read().await;
        let tag = self.tag.clone().unwrap();
        if tag.next.is_empty() {
            return Ok(());
        }
        match self.tag_repository.update_parent_by_ids(tag.next, 0).await {
            Ok(_) => Ok(()),
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    ctx.locale,
                    common::i18n::I18nKey::TagUpdateChild
                ));
            }
        }
    }
}
