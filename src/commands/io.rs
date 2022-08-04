use std::{fs::File, io::stdout, str::from_utf8};
use std::io::Write;
use anyhow::Result;
use fxread::{FastxRead, Record};

/// Matches the output to a writer stream
pub fn match_output_stream(output: Option<String>) -> Result<Box<dyn Write>>
{
    match output {
        Some(path) => Ok(Box::new(File::create(path)?)),
        None => Ok(Box::new(stdout()))
    }
}

/// Writes to the output stream with a provided closure
pub fn write_output(
    writer: &mut Box<dyn Write>, 
    reader: Box<dyn FastxRead<Item = Record>>,
    f: &dyn Fn(&Record) -> String)
{
    reader
        .map(|x| 
            if x.valid() { x } else { 
                panic!("Invalid Nucleotides in record: {:?}", from_utf8(x.id()).expect("invalid utf8")) 
            })
        .for_each(|x| {
            write!(writer, "{}", f(&x)).expect("Error Writing to File");
        });
}
