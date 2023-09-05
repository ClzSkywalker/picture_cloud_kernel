use common::{contextx::SharedStateCtx, errorx::Errorx};

use crate::query::{
    model::tag::dto::TagInfoItem, repository::tag_cqrs_repository::TagCqrsRepository,
};

pub struct TagCqrsService {
    pub ctx: SharedStateCtx,
    pub tag_cqrs_respository: TagCqrsRepository,
}

impl TagCqrsService {
    pub async fn find(&mut self) -> anyhow::Result<Vec<TagInfoItem>> {
        let ctx = self.ctx.read().await;
        let tags = match self.tag_cqrs_respository.find().await {
            Ok(r) => r,
            Err(_) => {
                anyhow::bail!(Errorx::new(ctx.locale, common::i18n::I18nKey::TagQuery))
            }
        };

        let mut res: Vec<TagInfoItem> = Vec::with_capacity(tags.len());
        for ele in tags {
            res.push(ele.into());
        }

        Ok(res)
    }
}
