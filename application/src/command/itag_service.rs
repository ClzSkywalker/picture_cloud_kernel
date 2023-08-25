use base::ddd::application_service::IApplicationService;

use crate::ability::tag::cmd::{tag_create_cmd::TagCreateCmd, tag_update_cmd::TagUpdateCmd};

#[async_trait::async_trait]
pub trait ITagService: IApplicationService {
    async fn create(&mut self, cmd: &TagCreateCmd) -> anyhow::Result<i32>;
    async fn update(&mut self, cmd: &TagUpdateCmd) -> anyhow::Result<()>;
}
