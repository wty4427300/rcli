mod cli;
mod process;
mod utils;

pub use cli::{Ops, Subcommands, Base64SubCommand, Base64Format, TextSignFormat, TextSubCommand, JwtKeyType, JwtSubCommand, HttpSubCommand};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}