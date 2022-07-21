use std::fs::File;
use std::io::Write;

use anyhow::Result;
use fxread::{initialize_reader, FastxRead, Record};

/// Writes results to stdout
fn write_to_stdout(reader: Box<dyn FastxRead<Item = Record>>) {
    reader
        .map(|x| 
            match x.valid() {
                true => { x }
                false => panic!("Invalid Nucleotides in record: {:?}", x)
            })
        .for_each(|x| {
            print!(">{}\n{}\n", x.id(), x.seq_rev_comp().unwrap())
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
            write!(file, ">{}\n{}\n", x.id(), x.seq_rev_comp().unwrap()).expect("Error Writing to File")
        });
    Ok(())
}

/// Runs reverse
pub fn run(
    input: String,
    output: Option<String>) -> Result<()> {

    let reader = initialize_reader(&input)?;
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
