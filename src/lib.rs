mod cli;
mod process;

pub use cli::{Ops, Subcommands,Base64SubCommand,Base64Format};
pub use process::{process_csv,process_genpass,process_encode, process_decode};
