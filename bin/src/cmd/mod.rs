use clap::{arg, Args, Parser, Subcommand};
use common::config::ServerMode;

pub mod api;

#[derive(Parser, Debug)]
#[command(author, version, about="picture_cloud", long_about = None)]
#[command(propagate_version = true)]
pub struct CliCommond {
    /// Starting the api Service
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Starting the api Service
    Api(ApiArgs),
}

#[derive(Debug, PartialEq, Args)]
pub struct ApiArgs {
    /// port
    #[arg(short, long, default_value = "6379")]
    pub port: i32,
    #[arg(short, long, default_value = "release")]
    pub mode: ServerMode,
    // #[arg(short, long, default_value = "/storage/emulated/0/Download")]
    // #[arg(short, long, default_value = "./workspace/todo_list.db")]
    // pub db_path: String,
    // #[arg(short, long, default_value = "./workspace/logs")]
    /// workspace
    #[arg(short, long, default_value = "./workspace")]
    pub workspace: String,
}
