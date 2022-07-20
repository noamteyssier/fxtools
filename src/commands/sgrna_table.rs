use anyhow::Result;
use spinners::{Spinner, Spinners};
use std::{collections::HashMap, fs::File, io::Write};
use fxread::{FastxRead, Record};

/// Creates a mapping of gene names to sgRNA names
struct Table {
    map: HashMap<String, String>
}
impl Table {
    
    /// creates a table from a [`FastxRead`] reader.
    pub fn from_reader(reader: Box<dyn FastxRead<Item = Record>>) -> Self  
    {
        let map = Self::build(reader);
        Self { map }
    }

    /// Returns the number of objects mapped
    pub fn num_records(&self) -> usize 
    {
        self.map.len()
    }

    /// main build iterator
    fn build(reader: Box<dyn FastxRead<Item = Record>>) -> HashMap<String, String>
    {
        reader
            .map(|record| (record.id().to_string(), Self::parse_header(&record)))
            .collect()
    }

    /// parses the gene name from the record header
    fn parse_header(record: &Record) -> String 
    {
        record.id().split('_').next().unwrap().to_string()
    }

    /// exposes a simple iterator function over the internal map
    fn iter(&self) -> impl Iterator<Item = (&String, &String)>{
        self.map.iter()
    }
}

/// Writes the table to stdout
fn write_to_stdout(
        table: Table, 
        delim: &str)
{
    table
        .iter()
        .for_each(|(k, v)| println!("{}{}{}", k, delim, v));
}

/// Writes the table to a file
fn write_to_file(
        table: Table, 
        path: &str, 
        delim: &str) -> Result<()>
{
    let mut file = File::create(path)?;
    table
        .iter()
        .for_each(
            |(k, v)| 
            writeln!(file, "{}{}{}", k, delim, v)
                .expect("Writing Error")
        );
    Ok(())
}

pub fn run(
    input: String,
    output: Option<String>,
    delim: Option<char>) -> Result<()> 
{
    let delim = match delim {
        Some(c) => c.to_string(),
        None => "\t".to_string()
    };
    let reader = fxread::initialize_reader(&input)?;

    let mut spinner = Spinner::new(Spinners::Dots12, "Determining Unique Records".to_string());
    let table = Table::from_reader(reader);
    spinner.stop_and_persist("✔", format!("Mapped {} Records", table.num_records()));
    match output {
        Some(f) => write_to_file(table, &f, &delim)?,
        None => write_to_stdout(table, &delim)
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use fxread::{FastxRead, Record, FastaReader, FastqReader};
    use super::Table;

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">AP2S1_ASJDAJSDAS\nACT\n>AP2S2_ASDKJASD\nACC\n>AP2S3_AOSDJIASJ\nACT\n";
        Box::new(FastaReader::new(sequence))
    }

    fn fastq_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b"@AP2S1_ASJDAJSDAS\nACT\n+\n123\n@AP2S2_ASDKJASD\nACC\n+\n123\n@AP2S3_AOSDJIASJ\nACT\n+\n123\n";
        Box::new(FastqReader::new(sequence))
    }

    #[test]
    fn table_fasta() {
        let reader = fasta_reader();
        let unique = Table::from_reader(reader);
        assert_eq!(unique.num_records(), 3);
        assert_eq!(unique.map.get("AP2S1_ASJDAJSDAS"), Some(&"AP2S1".to_string()));
        assert_eq!(unique.map.get("AP2S2_ASDKJASD"), Some(&"AP2S2".to_string()));
        assert_eq!(unique.map.get("AP2S3_AOSDJIASJ"), Some(&"AP2S3".to_string()));
    }

    #[test]
    fn table_fastq() {
        let reader = fastq_reader();
        let unique = Table::from_reader(reader);
        assert_eq!(unique.num_records(), 3);
        assert_eq!(unique.map.get("AP2S1_ASJDAJSDAS"), Some(&"AP2S1".to_string()));
        assert_eq!(unique.map.get("AP2S2_ASDKJASD"), Some(&"AP2S2".to_string()));
        assert_eq!(unique.map.get("AP2S3_AOSDJIASJ"), Some(&"AP2S3".to_string()));
    }
}
