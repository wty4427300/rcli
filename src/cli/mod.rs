mod base64;
mod csv;
mod genpass;
mod text;
mod jwt;
mod http;

use clap::Parser;
use std::path::{Path, PathBuf};
use crate::{CmdExecutor};
//使用self是为了不和create csv产生歧义
pub use self::{base64::*, csv::*, genpass::*, text::*, jwt::*, http::*};

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, long_about = None)]
pub struct Ops {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Parser, Debug)]
pub enum Subcommands {
    #[command(name = "csv", about = "Show CSV,Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "pass")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, name = "text", about = "text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "JWT encode/decode")]
    Jwt(JwtSubCommand),
    #[command(subcommand, about = "serve http server")]
    Http(HttpSubCommand),
}

impl CmdExecutor for Subcommands {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Subcommands::Base64(opts) => opts.execute().await,
            Subcommands::Text(opts) => opts.execute().await,
            Subcommands::Jwt(opts) => opts.execute().await,
            Subcommands::Http(opts) => opts.execute().await,
            Subcommands::Csv(opts) => opts.execute().await,
            Subcommands::GenPass(opts) => opts.execute().await,
        }
    }
}

fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File not found")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File not found"));
    }
}
