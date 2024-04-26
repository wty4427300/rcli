use clap::{Parser, Subcommand};
use std::path::Path;

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
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short,long,value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

pub fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File not found".into())
    }
}
