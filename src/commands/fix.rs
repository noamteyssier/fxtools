use std::io::stdin;

use super::{match_output_stream, write_mut_output_with_invalid};
use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};

/// Format prints the sequence as uppercase
fn format_print(record: &mut Record) -> &str {
    record.fix();
    record.as_str()
}

/// Runs uppercase
pub fn run(
    input: Option<String>,
    output: Option<String>,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;
    write_mut_output_with_invalid(&mut writer, reader, &format_print);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{format_print, write_mut_output_with_invalid};
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
    fn run_fasta_nochange() {
        let mut reader = fasta_reader();
        let mut record = reader.next().unwrap();
        let nochange = format_print(&mut record);
        assert_eq!(nochange, ">ap2s1_asjdajsdas\nact\n");
    }

    #[test]
    fn run_fastq_nochange() {
        let mut reader = fastq_reader();
        let mut record = reader.next().unwrap();
        let nochange = format_print(&mut record);
        assert_eq!(nochange, "@ap2s1_asjdajsdas\nact\n+\n123\n");
    }

    #[test]
    fn run_fasta() {
        let mut reader = invalid_fasta_reader();
        let mut record = reader.next().unwrap();
        let fixed = format_print(&mut record);
        assert_eq!(fixed, ">ap2s1_asjdajsdas\nNNN\n");
    }

    #[test]
    fn run_fastq() {
        let mut reader = invalid_fastq_reader();
        let mut record = reader.next().unwrap();
        let fixed = format_print(&mut record);
        assert_eq!(fixed, "@ap2s1_asjdajsdas\nNNN\n+\n123\n");
    }

    #[test]
    fn run_invalid_fasta_allow_invalid() {
        let reader = invalid_fasta_reader();
        let mut writer = File::create("/dev/null").unwrap();
        write_mut_output_with_invalid(&mut writer, reader, &format_print)
    }

    #[test]
    fn run_invalid_fastq_allow_invalid() {
        let reader = invalid_fastq_reader();
        let mut writer = File::create("/dev/null").unwrap();
        write_mut_output_with_invalid(&mut writer, reader, &format_print)
    }
}
