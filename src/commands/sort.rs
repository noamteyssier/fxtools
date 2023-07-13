use std::str::from_utf8;

use super::match_output_stream;
use anyhow::Result;
use fxread::{initialize_reader, Record};

fn prepare_record(record: &Record) -> String {
    if let Some(_) = record.qual() {
        format!(
            "@{}\n{}\n{}\n{}\n",
            from_utf8(record.id()).expect("invalid utf8"),
            from_utf8(record.seq()).expect("invalid utf8"),
            from_utf8(record.plus().unwrap()).expect("invalid utf8"),
            from_utf8(record.seq()).expect("invalid utf8"),
        )
    } else {
        format!(
            ">{}\n{}\n",
            from_utf8(record.id()).expect("invalid utf8"),
            from_utf8(record.seq()).expect("invalid utf8"),
        )
    }
}

fn write_pair<W>(writer_r1: &mut W, writer_r2: &mut W, records: &[(Record, Record)]) -> Result<()>
where
    W: std::io::Write,
{
    for (r1, r2) in records {
        let rec1 = prepare_record(r1);
        let rec2 = prepare_record(r2);
        write!(writer_r1, "{}", rec1)?;
        write!(writer_r2, "{}", rec2)?;
    }
    Ok(())
}

fn sort_paired_end(
    r1: &str,
    r2: &str,
    prefix: &str,
    gzip: bool,
    sort_by_r1: bool,
    num_threads: Option<usize>,
) -> Result<()> {
    // Determine output paths
    let mut output_r1 = format!("{prefix}_R1.fastq");
    let mut output_r2 = format!("{prefix}_R2.fastq");

    if gzip {
        output_r1.push_str(".gz");
        output_r2.push_str(".gz");
    }

    // Initialize paired readers
    let reader_r1 = initialize_reader(r1)?;
    let reader_r2 = initialize_reader(r2)?;

    // Zip paired readers into a single iterator and collect into a vector
    let mut records = reader_r1
        .into_iter()
        .zip(reader_r2.into_iter())
        .collect::<Vec<_>>();

    // Sort by sequence
    if sort_by_r1 {
        records.sort_by(|a, b| a.0.seq().cmp(b.0.seq()));
    } else {
        records.sort_by(|a, b| a.1.seq().cmp(b.1.seq()));
    }

    // Initialize writers
    let mut writer_r1 = match_output_stream(Some(output_r1), num_threads)?;
    let mut writer_r2 = match_output_stream(Some(output_r2), num_threads)?;

    // Write sorted records
    write_pair(&mut writer_r1, &mut writer_r2, &records)
}

fn sort_single_end(
    input: &str,
    prefix: &str,
    gzip: bool,
    num_threads: Option<usize>,
) -> Result<()> {
    // Determine output path
    let mut output = format!("{prefix}_R1.fastq");

    if gzip {
        output.push_str(".gz");
    }

    // Initialize reader
    let reader = initialize_reader(input)?;

    // Collect records into a vector
    let mut records = reader.into_iter().collect::<Vec<_>>();

    // Sort by sequence
    records.sort_by(|a, b| a.seq().cmp(b.seq()));

    // Initialize writer
    let mut writer = match_output_stream(Some(output), num_threads)?;

    // Write sorted records
    for record in records {
        let rec = prepare_record(&record);
        write!(writer, "{}", rec)?;
    }

    Ok(())
}

pub fn run(
    input: &str,
    r2: Option<String>,
    prefix: &str,
    gzip: bool,
    sort_by_r1: bool,
    num_threads: Option<usize>,
) -> Result<()> {
    if let Some(r2) = r2 {
        sort_paired_end(input, &r2, prefix, gzip, sort_by_r1, num_threads)
    } else {
        sort_single_end(input, prefix, gzip, num_threads)
    }
}
