use super::match_output_stream;
use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};
use regex::bytes::Regex;
use std::io::stdin;

pub fn match_regex(record: &Record, regex: &Regex, invert: bool, header: bool) -> bool {
    let pred = if header {
        regex.is_match(&record.id())
    } else {
        regex.is_match(&record.seq())
    };
    if invert {
        !pred
    } else {
        pred
    }
}

/// Runs Filtering
pub fn run(
    input: Option<String>,
    output: Option<String>,
    pattern: String,
    invert: bool,
    header: bool,
    num_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, num_threads, compression_level)?;
    let regex = Regex::new(&pattern)?;
    for record in reader {
        if match_regex(&record, &regex, invert, header) {
            write!(writer, "{}", record.as_str(),)?;
        }
    }
    Ok(())
}
