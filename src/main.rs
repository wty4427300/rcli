use clap::Parser;
use rcli::{process_csv, process_genpass, Ops, Subcommands};

fn main() -> anyhow::Result<()> {
    let opts = Ops::parse();
    match opts.cmd {
        Subcommands::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        Subcommands::GenPass(opts) => {
            let result = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            );
            match result {
                Ok(_) => {
                    println!("{:?}", result.expect("error"));
                }
                Err(_) => todo!(),
            }
        }
        Subcommands::Base64(_) => todo!(),
    }
    Ok(())
}
