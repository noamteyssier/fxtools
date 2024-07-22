use super::{io::match_input_stream, match_output_stream};
use anyhow::{anyhow, Result};
use clap::ValueEnum;
use csv::StringRecord;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Delimiter {
    Tab,
    Comma,
}
impl Delimiter {
    fn try_into(self) -> Result<u8> {
        match self {
            Self::Tab => Ok(b'\t'),
            Self::Comma => Ok(b','),
        }
    }
}

fn header_index(headers: &StringRecord, column: &str) -> Result<usize> {
    headers
        .iter()
        .position(|header| header == column)
        .ok_or(anyhow!(
            "Missing header name: {column}\nAvailable headers: [ {} ]",
            headers.iter().collect::<Vec<&str>>().join(", ")
        ))
}

pub fn run(
    input: Option<String>,
    output: Option<String>,
    header_col: String,
    sequence_col: String,
    delim: Delimiter,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    // Initializes the CSV reader
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delim.try_into()?)
        .from_reader(match_input_stream(input)?);

    // Get the headers and the index of the header columns
    let headers = reader.headers()?;
    let header_idx = header_index(headers, &header_col)?;
    let sequence_idx = header_index(headers, &sequence_col)?;

    // Initializes the FASTA writer
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;

    // Iterate through the CSV records and select the header and sequence columns
    for (idx, record) in reader.records().enumerate() {
        let record = record?;
        let header = record
            .get(header_idx)
            .ok_or(anyhow!("Missing header in row {idx}"))?;
        let sequence = record
            .get(sequence_idx)
            .ok_or(anyhow!("Missing sequence in row {idx}"))?;

        // Write the FASTA record
        writeln!(writer, ">{header}\n{sequence}")?;
    }
    Ok(())
}
