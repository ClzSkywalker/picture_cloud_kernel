use sea_orm::prelude::*;

use common::contextx::SharedStateCtx;
use infrastructure::db::model::preclude::*;
use sea_query::Condition;

pub struct TagCqrsRepository {
    pub ctx: SharedStateCtx,
}

impl TagCqrsRepository {
    pub async fn find(&self) -> anyhow::Result<Vec<TagInfoModel>> {
        let ctx = self.ctx.lock().await;
        let active = TagInfoEntity::find()
            .filter(Condition::all().add(Expr::col(TagInfoColumn::DeletedAt).is_null()));

        let res = match ctx.get_tx() {
            Some(r) => active.all(r).await,
            None => active.all(&ctx.db).await,
        };

        let tags = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{:?},e:{}", self.ctx, e);
                anyhow::bail!(e)
            }
        };
        Ok(tags)
    }
}
