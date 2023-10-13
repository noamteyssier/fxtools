use super::match_output_stream;
use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader};
use std::io::stdin;

pub fn run(
    input: Option<String>,
    output: Option<String>,
    num_records: usize,
    skip: usize,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;

    let record_iter = reader.skip(skip).take(num_records);
    for record in record_iter {
        write!(writer, "{}", record.as_str(),)?;
    }
    Ok(())
}
