use anyhow::Result;
use std::fs::File;
use std::io::Read;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    //可以使用ctrl+d发送eof符号
    #[test]
    fn test_get_reader() -> Result<()> {
        let mut reader = get_reader("-")?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        println!("{:?}", String::from_utf8(buf));
        Ok(())
    }
}
