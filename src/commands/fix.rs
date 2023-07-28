use super::{match_output_stream, write_mut_output};
use anyhow::Result;
use fxread::{initialize_reader, Record};
use std::str::from_utf8;

/// Format prints the sequence as uppercase
fn format_print(record: &mut Record) -> String {
    record.fix();
    match record.qual() {
        Some(_) => {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(record.seq()).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(record.qual().unwrap()).expect("invalid utf8"),
            )
        }
        None => {
            format!(
                ">{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(record.seq()).expect("invalid utf8")
            )
        }
    }
}

/// Runs uppercase
pub fn run(
    input: &str,
    output: Option<String>,
    num_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = initialize_reader(input)?;
    let mut writer = match_output_stream(output, num_threads, compression_level)?;
    write_mut_output(&mut writer, reader, &format_print);
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
    fn run_fasta_nochange() {
        let mut reader = fasta_reader();
        let nochange = reader.next().map(|ref mut x| format_print(x));
        assert_eq!(nochange, Some(">ap2s1_asjdajsdas\nact\n".to_string()));
    }

    #[test]
    fn run_fastq_nochange() {
        let mut reader = fastq_reader();
        let nochange = reader.next().map(|ref mut x| format_print(x));
        assert_eq!(
            nochange,
            Some("@ap2s1_asjdajsdas\nact\n+\n123\n".to_string())
        );
    }

    #[test]
    fn run_fasta() {
        let mut reader = invalid_fasta_reader();
        let fixed = reader.next().map(|ref mut x| format_print(x));
        assert_eq!(fixed, Some(">ap2s1_asjdajsdas\nNNN\n".to_string()));
    }

    #[test]
    fn run_fastq() {
        let mut reader = invalid_fastq_reader();
        let upper = reader.next().map(|ref mut x| format_print(x));
        assert_eq!(upper, Some("@ap2s1_asjdajsdas\nNNN\n+\n123\n".to_string()));
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
