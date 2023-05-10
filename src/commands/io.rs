use anyhow::Result;
use fxread::Record;
use std::borrow::Borrow;
use std::io::Write;
use std::{fs::File, io::stdout, str::from_utf8};

/// Matches the output to a writer stream
pub fn match_output_stream(output: Option<String>) -> Result<Box<dyn Write>> {
    match output {
        Some(path) => Ok(Box::new(File::create(path)?)),
        None => Ok(Box::new(stdout())),
    }
}

/// Writes to the output stream with a provided closure
pub fn write_output<W, I, R>(writer: &mut W, reader: I, f: &dyn Fn(&Record) -> String)
where
    W: Write,
    I: Iterator<Item = R>,
    R: Borrow<Record>,
{
    reader.for_each(|x| {
        assert!(
            x.borrow().valid(),
            "Invalid Nucleotides in record: {}",
            from_utf8(x.borrow().id()).expect("invalid utf8")
        );
        write!(writer, "{}", f(x.borrow())).expect("Error Writing to File");
    });
}
