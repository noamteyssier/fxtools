use std::{fs::File, io::stdout};
use std::io::Write;
use std::str::from_utf8;

use anyhow::Result;
use fxread::{initialize_reader, FastxRead, Record};

fn format_print(record: &Record) -> String {
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

/// Matches the output to a write stream
fn match_output(output: Option<String>) -> Result<Box<dyn Write>>
{
    match output {
        Some(path) => Ok(Box::new(File::create(path)?)),
        None => Ok(Box::new(stdout()))
    }
}

/// Writes results to file
fn write_output(writer: &mut Box<dyn Write>, reader: Box<dyn FastxRead<Item = Record>>) {
    reader
        .map(|x| 
            if x.valid() { x } else { 
                panic!("Invalid Nucleotides in record: {:?}", from_utf8(x.id()).expect("invalid utf8")) 
            })
        .for_each(|x| {
            write!(writer, "{}", format_print(&x)).expect("Error Writing to File");
        });
}

/// Runs uppercase
pub fn run(
    input: &str,
    output: Option<String>) -> Result<()> {

    let reader = initialize_reader(input)?;
    let mut writer = match_output(output)?;
    write_output(&mut writer, reader);
    Ok(())
}

#[cfg(test)]
mod test {
    use fxread::{FastxRead, Record, FastaReader, FastqReader};
    use super::{format_print, write_output, match_output};

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
        let mut reader = fasta_reader();
        let upper = reader.next().map(|x| format_print(&x));
        assert_eq!(upper, Some(">ap2s1_asjdajsdas\nACT\n".to_string()));
    }

    #[test]
    fn run_fastq() {
        let mut reader = fastq_reader();
        let upper = reader.next().map(|x| format_print(&x));
        assert_eq!(upper, Some("@ap2s1_asjdajsdas\nACT\n+\n123\n".to_string()));
    }

    #[test]
    #[should_panic]
    fn run_invalid_fasta() {
        let reader = invalid_fasta_reader();
        let mut writer = match_output(None).unwrap();
        write_output(&mut writer, reader)
    }

    #[test]
    #[should_panic]
    fn run_invalid_fastq() {
        let reader = invalid_fastq_reader();
        let mut writer = match_output(None).unwrap();
        write_output(&mut writer, reader)
    }
    
}
