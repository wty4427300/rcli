mod base64;
mod csv;
mod genpass;

use clap::Parser;

//使用self是为了不和create csv产生歧义
pub use self::{base64::*, csv::*, genpass::*};

#[derive(Parser, Debug)]
#[command(name="rcli",version,author,long_about=None)]
pub struct Ops {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    #[command(name = "csv", about = "Show CSV,Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "pass")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "base64")]
    Base64(Base64SubCommand),
}

fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if file_name = "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
