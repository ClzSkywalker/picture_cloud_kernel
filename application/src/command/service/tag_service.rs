use std::sync::Arc;

use base::ddd::{ability::IAbility, application_service::IApplicationService};
use common::contextx::AppContext;
use domain::aggregate::preclude::TagAggregate;

use crate::{ability::tag::cmd::TagCreateCmd, command::itag_service::ITagService};

pub struct TagService<CTA>
where
    CTA: IAbility<R = TagAggregate, CMD = TagCreateCmd>,
{
    pub ctx: Arc<AppContext>,
    pub tag_create_ability: CTA,
}

impl<CTA> IApplicationService for TagService<CTA> where
    CTA: IAbility<R = TagAggregate, CMD = TagCreateCmd>
{
}

#[async_trait::async_trait]
impl<CTA> ITagService for TagService<CTA>
where
    CTA: IAbility<R = TagAggregate, CMD = TagCreateCmd>,
{
    async fn create(&mut self, cmd: &TagCreateCmd) -> anyhow::Result<i32> {
        let tag = match __self.tag_create_ability.execute_ability(cmd).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };
        Ok(tag.id)
    }
}
