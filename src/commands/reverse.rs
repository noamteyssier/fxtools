use std::io::stdin;

use super::{match_output_stream, write_mut_output};
use anyhow::Result;
use fxread::{initialize_reader, Record, initialize_stdin_reader};

/// Reverse complement sequence and create a string representation of the record
fn format_print(record: &mut Record) -> &str {
    record.rev_comp();
    record.as_str()
}

/// Runs reverse
pub fn run(
    input: Option<String>,
    output: Option<String>,
    num_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, num_threads, compression_level)?;
    write_mut_output(&mut writer, reader, &format_print);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{format_print, match_output_stream, write_mut_output};
    use fxread::{FastaReader, FastqReader, FastxRead, Record};

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
        let rev = format_print(&mut record);
        assert_eq!(rev, ">ap2s1_asjdajsdas\nagt\n");
    }

    #[test]
    fn run_fastq() {
        let mut reader = fastq_reader();
        let mut record = reader.next().unwrap();
        let rev = format_print(&mut record);
        assert_eq!(rev, "@ap2s1_asjdajsdas\nagt\n+\n321\n");
    }

    #[test]
    #[should_panic]
    fn run_invalid_fasta() {
        let reader = invalid_fasta_reader();
        let mut writer = match_output_stream(None, None, None).unwrap();
        write_mut_output(&mut writer, reader, &format_print)
    }

    #[test]
    #[should_panic]
    fn run_invalid_fastq() {
        let reader = invalid_fastq_reader();
        let mut writer = match_output_stream(None, None, None).unwrap();
        write_mut_output(&mut writer, reader, &format_print)
    }
}
