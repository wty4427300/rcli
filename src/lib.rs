mod cli;
mod process;

pub use cli::{Ops, Subcommands,Base64SubCommand};
pub use process::{process_csv,process_genpass};
