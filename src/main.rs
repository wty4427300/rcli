use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_decrypt, process_text_encrypt, process_text_key_generate, process_text_sign,
    process_text_verify, process_jwt_decode, process_jwt_encode, Base64SubCommand, Ops, Subcommands, TextSubCommand, JwtSubCommand,
};
use std::fs;

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
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.output, opts.format)?;
                // TODO: decoded data might not be string (but for this example, we assume it is)
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        Subcommands::Text(cmd) => match cmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                // base64 output
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
            TextSubCommand::Decrypt(opts) => {
                let decrypted = process_text_encrypt(&opts.input, &opts.key)?;
                println!("{}", decrypted);
            }
            TextSubCommand::Encrypt(opts) => {
                let encrypted = process_text_decrypt(&opts.input, &opts.key)?;
                println!("{}", encrypted);
            }
        },
        Subcommands::Jwt(cmd) => {
            match cmd {
                JwtSubCommand::Sign(opts) => {
                    let token = process_jwt_encode(&opts.key, opts.key_type, &opts.sub, &opts.aud, opts.exp, opts.algorithm)?;
                    println!("{}", token);
                }
                JwtSubCommand::Verify(opts) => {
                    let claim = process_jwt_decode(&opts.key, opts.key_type, &opts.token, opts.algorithm, &opts.aud)?;
                    println!("sub: {}, aud: {}, exp: {}", claim.sub, claim.aud, claim.exp);
                }
            }
        }
    }
    Ok(())
}
