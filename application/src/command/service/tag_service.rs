use base::ddd::{ability::IAbility, application_service::IApplicationService};
use domain::aggregate::preclude::TagAggregate;

use crate::{
    ability::tag::cmd::{tag_create_cmd::TagCreateCmd, tag_update_cmd::TagUpdateCmd},
    command::itag_service::ITagService,
};

pub struct TagService<TCA, TUA>
where
    TCA: IAbility<R = TagAggregate, CMD = TagCreateCmd>,
    TUA: IAbility<R = TagAggregate, CMD = TagUpdateCmd>,
{
    pub tag_create_ability: TCA,
    pub tag_update_ability: TUA,
}

impl<TCA, TUA> IApplicationService for TagService<TCA, TUA>
where
    TCA: IAbility<R = TagAggregate, CMD = TagCreateCmd>,
    TUA: IAbility<R = TagAggregate, CMD = TagUpdateCmd>,
{
}

#[async_trait::async_trait]
impl<TCA, TUA> ITagService for TagService<TCA, TUA>
where
    TCA: IAbility<R = TagAggregate, CMD = TagCreateCmd>,
    TUA: IAbility<R = TagAggregate, CMD = TagUpdateCmd>,
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

    async fn update(&mut self, cmd: &TagUpdateCmd) -> anyhow::Result<()> {
        match __self.tag_update_ability.execute_ability(cmd).await {
            Ok(_) => Ok(()),
            Err(e) => {
                anyhow::bail!(e)
            }
        }
    }
}
