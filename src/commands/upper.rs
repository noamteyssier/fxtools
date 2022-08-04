use std::fs::File;
use std::io::Write;
use std::str::from_utf8;

use anyhow::Result;
use fxread::{initialize_reader, FastxRead, Record};

fn format_print(record: Record) -> String {
    match record.qual() {
        Some(_) => {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq_upper()).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(record.qual().unwrap()).expect("invalid utf8"),
                )
        },
        None => {
            format!(
                ">{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq_upper()).expect("invalid utf8")
                )
        }
    }
}

/// Writes results to stdout
fn write_to_stdout(reader: Box<dyn FastxRead<Item = Record>>) {
    reader
        .map(|x| 
            match x.valid() {
                true => { x }
                false => panic!("Invalid Nucleotides in record: {:?}", x)
            })
        .for_each(|x| {
            print!("{}", format_print(x))
        })
}

/// Writes results to file
fn write_to_file(output: &str, reader: Box<dyn FastxRead<Item = Record>>) -> Result<()> {
    let mut file = File::create(output)?;
    reader
        .map(|x| 
            match x.valid() {
                true => { x }
                false => panic!("Invalid Nucleotides in record: {:?}", x)
            })
        .for_each(|x| {
            write!(file, "{}", format_print(x)).expect("Error Writing to File")
        });
    Ok(())
}

/// Runs uppercase
pub fn run(
    input: &str,
    output: Option<String>) -> Result<()> {

    let reader = initialize_reader(input)?;
    match output {
        Some(s) => write_to_file(&s, reader)?,
        None => write_to_stdout(reader)
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use fxread::{FastxRead, Record, FastaReader, FastqReader};
    use super::write_to_stdout;

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">ap2s1_asjdajsdas\nact\n>ap2s1_asdkjasd\nacc\n>ap2s2_aosdjiasj\nact\n";
        Box::new(FastaReader::new(sequence))
    }

    fn invalid_fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">ap2s1_asjdajsdas\nbrb\n>ap2s1_asdkjasd\nacc\n>ap2s2_aosdjiasj\nact\n";
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
        let reader = fasta_reader();
        write_to_stdout(reader);
    }

    #[test]
    fn run_fastq() {
        let reader = fastq_reader();
        write_to_stdout(reader);
    }

    #[test]
    #[should_panic]
    fn run_invalid_fasta() {
        let reader = invalid_fasta_reader();
        write_to_stdout(reader)
    }

    #[test]
    #[should_panic]
    fn run_invalid_fastq() {
        let reader = invalid_fastq_reader();
        write_to_stdout(reader)
    }
    
}
