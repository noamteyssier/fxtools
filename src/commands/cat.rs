use std::io::Write;

use anyhow::Result;
use fxread::{initialize_reader, Record};

use super::match_output_stream;

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

/// Runs the `cat` command.
pub fn run(
    inputs: Vec<String>,
    output: Option<String>,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;
    for filename in inputs {
        let reader = initialize_reader(&filename)?;
        write_file(reader, &mut writer)?;
    }
    Ok(())
}
