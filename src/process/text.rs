use crate::{get_reader, TextSignFormat};
use anyhow::Result;

trait TextSign {
    fn sign(&self,key:&str)->Result<String>;
}

pub fn process_sign(input: &str,key:&str,format:TextSignFormat)->Result<()>{
    let mut reader=get_reader(input)?;
    let mut buf=Vec::new();
    reader.read_to_end(&mut buf)?;
    let signed=match format {
        TextSignFormat::Blake3 => todo!(),
        TextSignFormat::Ed25519 => todo!(),
    };
    println!("{}",signed);
    Ok(())
}