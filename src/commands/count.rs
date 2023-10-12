use std::io::stdin;

use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};

/// Counts the records
pub fn count<R: Iterator<Item = Record>>(input: R) -> usize {
    input.count()
}

/// Runs the `count` command.
pub fn run(input: Option<String>) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let num_records = count(reader);
    println!("{}", num_records);
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
    fn test_count_fasta() {
        let reader = fasta_reader();
        let num_records = count(reader);
        assert_eq!(num_records, 3);
    }

    #[test]
    fn test_count_fastq() {
        let reader = fastq_reader();
        let num_records = count(reader);
        assert_eq!(num_records, 3);
    }
}
