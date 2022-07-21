use std::fs::File;
use std::io::Write;

use anyhow::Result;
use fxread::{initialize_reader, FastxRead, Record};

/// Writes results to stdout
fn write_to_stdout(reader: Box<dyn FastxRead<Item = Record>>) {
    reader
        .map(|x| 
            match x.valid() {
                true => { x }
                false => panic!("Invalid Nucleotides in record: {:?}", x)
            })
        .for_each(|x| {
            print!(">{}\n{}\n", x.id(), x.seq_upper())
        })
}

/// Writes results to file
fn write_to_file(output: &str, reader: Box<dyn FastxRead<Item = Record>>) -> Result<()> {
    let mut file = File::create(output)?;
    reader
        .map(|x| 
            match x.valid() {
                true => { x }
                false => panic!("Invalid Nucleotides in record: {:?}", x)
            })
        .for_each(|x| {
            write!(file, ">{}\n{}\n", x.id(), x.seq_upper()).expect("Error Writing to File")
        });
    Ok(())
}

/// Runs uppercase
pub fn run(
    input: String,
    output: Option<String>) -> Result<()> {

    let reader = initialize_reader(&input)?;
    match output {
        Some(s) => write_to_file(&s, reader)?,
        None => write_to_stdout(reader)
    };
    Ok(())
}
