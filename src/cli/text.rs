use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use super::{verify_file, verify_path};

#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(about = "sign a message with a private/shared key")]
    Sign(TextSingOpts),
    #[command(about = "verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(KeyGenerateOpts),
    #[command(about = "Encrypt a message use cha-cha20-poly1305.")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message use cha-cha20-poly1305.")]
    Decrypt(TextDecryptOpts),
}

#[derive(Parser, Debug)]
pub struct TextSingOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}


#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

//文本转化成枚举
impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

//枚举转化成文本
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

//打印
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
