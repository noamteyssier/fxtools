use super::{match_output_stream, write_output};
use anyhow::Result;
use fxread::{initialize_reader, Record};
use std::str::from_utf8;

/// Reverse complement sequence and create a string representation of the record
fn format_print(record: &Record) -> String {
    match record.qual() {
        Some(_) => {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq_rev_comp()).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(record.qual().unwrap()).expect("invalid utf8"),
            )
        }
        None => {
            format!(
                ">{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq_rev_comp()).expect("invalid utf8")
            )
        }
    }
}

/// Runs reverse
pub fn run(input: &str, output: Option<String>, num_threads: Option<usize>) -> Result<()> {
    let reader = initialize_reader(input)?;
    let mut writer = match_output_stream(output, num_threads)?;
    write_output(&mut writer, reader, &format_print);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{format_print, match_output_stream, write_output};
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
        let rev = reader.next().map(|x| format_print(&x));
        assert_eq!(rev, Some(">ap2s1_asjdajsdas\nagt\n".to_string()));
    }

    #[test]
    fn run_fastq() {
        let mut reader = fastq_reader();
        let rev = reader.next().map(|x| format_print(&x));
        assert_eq!(rev, Some("@ap2s1_asjdajsdas\nagt\n+\n123\n".to_string()));
    }

    #[test]
    #[should_panic]
    fn run_invalid_fasta() {
        let reader = invalid_fasta_reader();
        let mut writer = match_output_stream(None, None).unwrap();
        write_output(&mut writer, reader, &format_print)
    }

    #[test]
    #[should_panic]
    fn run_invalid_fastq() {
        let reader = invalid_fastq_reader();
        let mut writer = match_output_stream(None, None).unwrap();
        write_output(&mut writer, reader, &format_print)
    }
}
