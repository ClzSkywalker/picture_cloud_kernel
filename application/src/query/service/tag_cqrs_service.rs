use common::{contextx::AppContext, errorx::Errorx};
use std::sync::Arc;

use crate::query::{
    model::tag::dto::TagInfoItem, repository::tag_cqrs_repository::TagCqrsRepository,
};

pub struct TagCqrsService {
    pub ctx: Arc<AppContext>,
    pub tag_cqrs_respository: TagCqrsRepository,
}

impl TagCqrsService {
    pub async fn find(&mut self) -> anyhow::Result<Vec<TagInfoItem>> {
        let tags = match self.tag_cqrs_respository.find().await {
            Ok(r) => r,
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TagQuery
                ))
            }
        };

        let mut res: Vec<TagInfoItem> = Vec::with_capacity(tags.len());
        for ele in tags {
            res.push(ele.into());
        }

        Ok(res)
    }
}
