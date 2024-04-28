use anyhow;
use clap::Parser;
use rcli::{process_csv, Ops, Subcommands};

fn main() -> anyhow::Result<()> {
    let opts = Ops::parse();
    match opts.cmd {
        Subcommands::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}",opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
    }
    Ok(())
}
