use super::match_output_stream;
use anyhow::Result;
use fxread::{initialize_reader, Record};
use std::io::Write;

fn write_file<R, W>(reader: R, writer: &mut W) -> Result<()>
where
    R: Iterator<Item = Record>,
    W: Write,
{
    for record in reader {
        write!(writer, "{}", record.as_str())?;
    }
    Ok(())
}

fn write_sequences<R, W>(reader: R, writer: &mut W, single_line: bool) -> Result<()>
where
    R: Iterator<Item = Record>,
    W: Write,
{
    for record in reader {
        if single_line {
            write!(writer, "{}", record.seq_str())?;
        } else {
            write!(writer, "{}\n", record.seq_str())?;
        }
    }
    Ok(())
}

fn write_headers<R, W>(reader: R, writer: &mut W) -> Result<()>
where
    R: Iterator<Item = Record>,
    W: Write,
{
    for record in reader {
        write!(writer, "{}\n", record.id_str())?;
    }
    Ok(())
}

/// Runs the `cat` command.
pub fn run(
    inputs: Vec<String>,
    output: Option<String>,
    sequence_only: bool,
    headers_only: bool,
    single_line: bool,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;
    for filename in inputs {
        let reader = initialize_reader(&filename)?;
        if sequence_only {
            write_sequences(reader, &mut writer, single_line)?;
        } else if headers_only {
            write_headers(reader, &mut writer)?;
        } else {
            write_file(reader, &mut writer)?;
        }
    }
    Ok(())
}
