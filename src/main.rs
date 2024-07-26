use clap::Parser;
use rcli::{CmdExecutor, Ops};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Ops::parse();
    opts.cmd.execute().await?;
    Ok(())
}
