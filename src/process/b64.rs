use base64::prelude::*;

pub fn process_encode(intput: &str) -> anyhow::Result<()> {
    let encode = BASE64_STANDARD.encode(intput);
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(output: &str) -> anyhow::Result<()> {
    let decode = BASE64_STANDARD.decode(output)?;
    println!("{}", String::from_utf8(decode)?);
    Ok(())
}
