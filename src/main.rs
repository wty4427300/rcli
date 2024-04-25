use std::path::Path;
use clap::{Parser,Subcommand};
use csv::Reader;
use serde::{Deserialize,Serialize};
use anyhow;

#[derive(Parser,Debug)]
#[command(name="rcli",version,author,long_about=None)]
struct Ops{
    #[command(subcommand)]
    cmd:Subcommands,
}

#[derive(Subcommand,Debug)]
enum Subcommands{
    #[command(name="csv",about="Show CSV,Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Parser,Debug)]
struct CsvOpts{
    #[arg(short,long,value_parser = verify_input_file)]
    input:String,
    #[arg(short,long,default_value = "output.json")]
    output:String,
    #[arg(short,long,default_value_t = ',')]
    delimiter:char,
    #[arg(long,default_value_t=true)]
    header:bool,
}

#[derive(Debug,Deserialize,Serialize)]
struct Player{
    #[serde(rename="Name")]
    name:String,
    #[serde(rename="Position")]
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    #[serde(rename="Nationality")]
    nationality:String,
    #[serde(rename="Kit Number")]
    kit:u8,

}

pub fn verify_input_file(file_name:&str)->Result<String,String>{
    if Path::new(file_name).exists(){
        Ok(file_name.into())
    } else {
        Err("File not found".into())   
    }
}

fn main()->anyhow::Result<()> {
    let opts=Ops::parse();
    match opts.cmd {
         Subcommands::Csv(opts)=>{
            let mut reader=Reader::from_path(opts.input)?;
            for result in reader.deserialize(){
                let player:Player=result?;
                println!("{:?}",player);
            }
         }  
    }
    Ok(())
}
