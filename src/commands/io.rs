use anyhow::Result;
use flate2::read::MultiGzDecoder;
use fxread::Record;
use gzp::deflate::Gzip;
use gzp::par::compress::{ParCompress, ParCompressBuilder};
use gzp::Compression;
use std::borrow::{Borrow, BorrowMut};
use std::io::{stdin, BufReader, Read, Write};
use std::{fs::File, io::stdout, str::from_utf8};

/// Matches the input to a reader stream
pub fn match_input_stream(input: Option<String>) -> Result<Box<dyn Read>> {
    match input {
        Some(path) => {
            if path.ends_with(".gz") {
                let file = File::open(path)?;
                let buffer = BufReader::new(file);
                let reader = MultiGzDecoder::new(buffer);
                Ok(Box::new(reader))
            } else {
                let file = File::open(path)?;
                let buffer = BufReader::new(file);
                Ok(Box::new(buffer))
            }
        }
        None => Ok(Box::new(BufReader::new(stdin()))),
    }
}

/// Matches the output to a writer stream
pub fn match_output_stream(
    output: Option<String>,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<Box<dyn Write>> {
    match output {
        Some(path) => {
            if path.ends_with(".gz") {
                let file = File::create(path)?;
                let writer: ParCompress<Gzip> = ParCompressBuilder::new()
                    .num_threads(compression_threads.unwrap_or(1))?
                    .compression_level(if let Some(level) = compression_level {
                        Compression::new(level as u32)
                    } else {
                        Compression::default()
                    })
                    .from_writer(file);
                Ok(Box::new(writer))
            } else {
                Ok(Box::new(File::create(path)?))
            }
        }
        None => Ok(Box::new(stdout())),
    }
}

/// Writes to the output stream with a provided closure
pub fn write_output<W, I, R>(writer: &mut W, reader: I, f: &dyn Fn(&Record) -> &str)
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

/// Writes to the output stream with a provided closure that mutates the record
pub fn write_mut_output<W, I, R>(writer: &mut W, reader: I, f: &dyn Fn(&mut Record) -> &str)
where
    W: Write,
    I: Iterator<Item = R>,
    R: BorrowMut<Record>,
{
    reader.for_each(|mut x| {
        assert!(
            x.borrow().valid(),
            "Invalid Nucleotides in record: {}",
            from_utf8(x.borrow().id()).expect("invalid utf8")
        );
        write!(writer, "{}", f(x.borrow_mut())).expect("Error Writing to File");
    });
}

/// Writes to the output stream with a provided closure
/// but does not check for valid nucleotides
pub fn write_output_with_invalid<W, I, R>(writer: &mut W, reader: I, f: &dyn Fn(&Record) -> &str)
where
    W: Write,
    I: Iterator<Item = R>,
    R: Borrow<Record>,
{
    reader.for_each(|x| {
        write!(writer, "{}", f(x.borrow())).expect("Error Writing to File");
    });
}

/// Writes to the output stream with a provided closure that mutates the record
/// but does not check for valid nucleotides
pub fn write_mut_output_with_invalid<W, I, R>(
    writer: &mut W,
    reader: I,
    f: &dyn Fn(&mut Record) -> &str,
) where
    W: Write,
    I: Iterator<Item = R>,
    R: BorrowMut<Record>,
{
    reader.for_each(|mut x| {
        write!(writer, "{}", f(x.borrow_mut())).expect("Error Writing to File");
    });
}
