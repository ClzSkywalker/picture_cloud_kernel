use base::ddd::application_service::IApplicationService;

use crate::ability::tag::cmd::TagCreateCmd;

#[async_trait::async_trait]
pub trait ITagService: IApplicationService {
    async fn create(&mut self, cmd: &TagCreateCmd) -> anyhow::Result<i32>;
}
