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
    async fn check_handler(&mut self, cmd: &Self::CMD) -> anyhow::Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 能力点执行前的幂等校验,当前能力点已执行，不再执行业务逻辑、true：当前能力点未执行，继续执行业务逻辑
    /// param           {*} self
    /// return          {*}
    ///    
    async fn check_idempotent(&mut self, cmd: &Self::CMD) -> anyhow::Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 执行
    /// param           {*} self
    /// return          {*}
    ///    
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R>;

    async fn execute_ability(&mut self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        match self.check_handler(cmd).await {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!(e)
            }
        };
        match self.check_idempotent(cmd).await {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!(e)
            }
        };
        self.execute(cmd).await
    }
}
