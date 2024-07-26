use std::path::PathBuf;
use clap::Parser;
use crate::{CmdExecutor, process_http_server};
use super::verify_path;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve HTTP Server")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8081")]
    pub port: u16,
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => {
                process_http_server(opts.dir, opts.port).await
            }
        }
    }
}
