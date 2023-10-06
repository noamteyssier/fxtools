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
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;
    let regex = Regex::new(&pattern)?;
    for record in reader {
        if match_regex(&record, &regex, invert, header) {
            write!(writer, "{}", record.as_str(),)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use fxread::{FastaReader, FastqReader, FastxRead, Record};

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] =
            b">ap2s1_asjdajsdas\nact\n>ap2s1_asdkjasd\nacc\n>ap2s2_aosdjiasj\nact\n";
        Box::new(FastaReader::new(sequence))
    }

    fn fastq_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b"@ap2s1_asjdajsdas\nact\n+\n123\n@ap2s1_asdkjasd\nacc\n+\n123\n@ap2s2_aosdjiasj\nact\n+\n123\n";
        Box::new(FastqReader::new(sequence))
    }

    #[test]
    fn test_fasta_sequence() {
        let reader = fasta_reader();
        let regex = Regex::new("act").unwrap();
        let invert = false;
        let header = false;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 2);
    }

    #[test]
    fn test_fasta_header() {
        let reader = fasta_reader();
        let regex = Regex::new("ap2s1").unwrap();
        let invert = false;
        let header = true;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 2);
    }

    #[test]
    fn test_fasta_sequence_inverse() {
        let reader = fasta_reader();
        let regex = Regex::new("act").unwrap();
        let invert = true;
        let header = false;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 1);
    }

    #[test]
    fn test_fasta_header_inverse() {
        let reader = fasta_reader();
        let regex = Regex::new("ap2s1").unwrap();
        let invert = true;
        let header = true;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 1);
    }

    #[test]
    fn test_fastq_sequence() {
        let reader = fastq_reader();
        let regex = Regex::new("act").unwrap();
        let invert = false;
        let header = false;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 2);
    }

    #[test]
    fn test_fastq_header() {
        let reader = fastq_reader();
        let regex = Regex::new("ap2s1").unwrap();
        let invert = false;
        let header = true;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 2);
    }

    #[test]
    fn test_fastq_sequence_inverse() {
        let reader = fastq_reader();
        let regex = Regex::new("act").unwrap();
        let invert = true;
        let header = false;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 1);
    }

    #[test]
    fn test_fastq_header_inverse() {
        let reader = fastq_reader();
        let regex = Regex::new("ap2s1").unwrap();
        let invert = true;
        let header = true;
        let matches = reader
            .filter(|x| match_regex(x, &regex, invert, header))
            .count();
        assert_eq!(matches, 1);
    }
}
