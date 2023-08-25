use clap::Parser;
use cmd::{api::server_api, CliCommond};
use common::config::AppConfig;

pub mod cmd;

#[tokio::main]
async fn main() {
    let cli = CliCommond::parse();

    match cli.command {
        cmd::Commands::Api(args) => {
            let mut config = AppConfig::default();
            config.mode = args.mode;
            config.port = args.port;
            config.workspace = args.workspace;
            config.version = common::config::VERSION.to_string();
            config.init();
            server_api(config).await
        }
    }
}
