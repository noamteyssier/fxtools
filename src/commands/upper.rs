use std::io::stdin;

use super::{match_output_stream, write_mut_output, write_mut_output_with_invalid};
use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};

/// Format prints the sequence as uppercase
fn format_print(record: &mut Record) -> &str {
    record.upper();
    record.as_str()
}

/// Runs uppercase
pub fn run(
    input: Option<String>,
    output: Option<String>,
    num_threads: Option<usize>,
    compression_level: Option<usize>,
    allow_invalid: bool,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, num_threads, compression_level)?;
    if allow_invalid {
        write_mut_output_with_invalid(&mut writer, reader, &format_print);
    } else {
        write_mut_output(&mut writer, reader, &format_print);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{format_print, match_output_stream, write_mut_output};
    use crate::commands::io::write_mut_output_with_invalid;
    use fxread::{FastaReader, FastqReader, FastxRead, Record};
    use std::fs::File;

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] =
            b">ap2s1_asjdajsdas\nact\n>ap2s1_asdkjasd\nacc\n>ap2s2_aosdjiasj\nact\n";
        Box::new(FastaReader::new(sequence))
    }

    fn invalid_fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] =
            b">ap2s1_asjdajsdas\nbrb\n>ap2s1_asdkjasd\nacc\n>ap2s2_aosdjiasj\nact\n";
        Box::new(FastaReader::new(sequence))
    }

    fn fastq_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b"@ap2s1_asjdajsdas\nact\n+\n123\n@ap2s1_asdkjasd\nacc\n+\n123\n@ap2s2_aosdjiasj\nact\n+\n123\n";
        Box::new(FastqReader::new(sequence))
    }

    fn invalid_fastq_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b"@ap2s1_asjdajsdas\nbrb\n+\n123\n@ap2s1_asdkjasd\nacc\n+\n123\n@ap2s2_aosdjiasj\nact\n+\n123\n";
        Box::new(FastqReader::new(sequence))
    }

    #[test]
    fn run_fasta() {
        let mut reader = fasta_reader();
        let mut record = reader.next().unwrap();
        let upper = format_print(&mut record);
        assert_eq!(upper, ">ap2s1_asjdajsdas\nACT\n");
    }

    #[test]
    fn run_fastq() {
        let mut reader = fastq_reader();
        let mut record = reader.next().unwrap();
        let upper = format_print(&mut record);
        assert_eq!(upper, "@ap2s1_asjdajsdas\nACT\n+\n123\n");
    }

    #[test]
    #[should_panic]
    fn run_invalid_fasta() {
        let reader = invalid_fasta_reader();
        let mut writer = match_output_stream(None, None, None).unwrap();
        write_mut_output(&mut writer, reader, &format_print)
    }

    #[test]
    fn run_invalid_fasta_allow_invalid() {
        let reader = invalid_fasta_reader();
        let mut writer = File::create("/dev/null").unwrap();
        write_mut_output_with_invalid(&mut writer, reader, &format_print)
    }

    #[test]
    #[should_panic]
    fn run_invalid_fastq() {
        let reader = invalid_fastq_reader();
        let mut writer = match_output_stream(None, None, None).unwrap();
        write_mut_output(&mut writer, reader, &format_print)
    }

    #[test]
    fn run_invalid_fastq_allow_invalid() {
        let reader = invalid_fastq_reader();
        let mut writer = File::create("/dev/null").unwrap();
        write_mut_output_with_invalid(&mut writer, reader, &format_print)
    }
}
