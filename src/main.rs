use anyhow;
use clap::Parser;
use rcli::{process_csv, Ops, Subcommands};

fn main() -> anyhow::Result<()> {
    let opts = Ops::parse();
    match opts.cmd {
        Subcommands::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
