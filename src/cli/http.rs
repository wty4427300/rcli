use std::path::PathBuf;
use clap::Parser;

use super::verify_path;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve HTTP Server")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
