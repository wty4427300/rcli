use base64::prelude::*;
use crate::Base64Format;
use std::fs::File;
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(buf),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(output: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if output == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(output)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(output)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(output)?,
    };
    println!("{}", String::from_utf8(decode)?);
    Ok(())
}
