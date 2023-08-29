use common::contextx::AppContext;

use super::command::ICommand;

#[async_trait::async_trait]
pub trait IAbility: Send + Sync {
    /// result
    type R;
    type CMD: ICommand;
    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 能力点执行前的参数校验
    /// param           {*} self
    /// return          {*}
    ///    
    async fn check_handler(&mut self, ctx: &mut AppContext, cmd: &Self::CMD) -> anyhow::Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 能力点执行前的幂等校验,当前能力点已执行，不再执行业务逻辑、true：当前能力点未执行，继续执行业务逻辑
    /// param           {*} self
    /// return          {*}
    ///    
    async fn check_idempotent(
        &mut self,
        ctx: &mut AppContext,
        cmd: &Self::CMD,
    ) -> anyhow::Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 执行
    /// param           {*} self
    /// return          {*}
    ///    
    async fn execute(&mut self, ctx: &mut AppContext, cmd: &Self::CMD) -> anyhow::Result<Self::R>;

    async fn execute_ability(
        &mut self,
        ctx: &mut AppContext,
        cmd: &Self::CMD,
    ) -> anyhow::Result<Self::R> {
        match self.check_handler(ctx, cmd).await {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!(e)
            }
        };
        match self.check_idempotent(ctx, cmd).await {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!(e)
            }
        };
        self.execute(ctx, cmd).await
    }
}
