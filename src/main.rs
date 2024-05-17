use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Ops,
    Subcommands,
};

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
            )?;
            println!("{:?}", result);
        }
        Subcommands::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.output, opts.format)?;
            }
        },
        Subcommands::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opts) => {
                println!("Sign: {:?}", opts);
            }
            rcli::TextSubCommand::Verify(opts) => {
                println!("Verify: {:?}", opts);
            }
        },
    }
    Ok(())
}
