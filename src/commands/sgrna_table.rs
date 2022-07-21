use anyhow::Result;
use spinners::{Spinner, Spinners};
use std::{collections::HashMap, fs::File, io::Write};
use fxread::{FastxRead, Record};

/// Creates a mapping of gene names to sgRNA names
struct Table {
    map: HashMap<String, Vec<Record>>
}
impl Table {
    
    /// creates a table from a [`FastxRead`] reader.
    pub fn from_reader(reader: Box<dyn FastxRead<Item = Record>>) -> Self  
    {
        let map = Self::build(reader);
        Self { map }
    }
    
    /// Returns the number of genes found
    pub fn num_genes(&self) -> usize
    {
        self.map.len()
    }

    /// Returns the number of records mapped
    pub fn num_records(&self) -> usize 
    {
        self.map.values().flatten().count()
    }

    /// Write table to stdout
    pub fn write_to_stdout(&self, delim: &str, include_sequence: bool) {
        self.map
            .iter()
            .for_each(
                |(k, v)| 
                v.iter().for_each(|record| {
                    println!("{}", self.prepare_result(k, delim, record, include_sequence))
                })
            );
    }

    /// Write table to file
    pub fn write_to_file(&self, path: &str, delim: &str, include_sequence: bool) -> Result<()> {
        let mut file = File::create(path)?;
        self.map
            .iter()
            .for_each(
                |(k, v)| 
                v.iter().for_each(|record| {
                    writeln!(file, "{}", self.prepare_result(k, delim, record, include_sequence))
                        .expect("Writing Error")
                })
            );
        Ok(())
    }

    /// Properly formats the string for output
    fn prepare_result(&self, gene: &str, delim: &str, record: &Record, include_sequence: bool) -> String {
        match include_sequence {
            true => format!("{}{}{}{}{}", gene, delim, record.id(), delim, record.seq()),
            false => format!("{}{}{}", gene, delim, record.id())
        }

    }

    /// main build iterator
    fn build(reader: Box<dyn FastxRead<Item = Record>>) -> HashMap<String, Vec<Record>>
    {
        reader
            .fold(
                HashMap::new(),
                |mut table, record| {
                    let gene = Self::parse_header(&record);
                    table.entry(gene).or_insert(Vec::new()).push(record);
                    table
                })
    }

    /// parses the gene name from the record header
    fn parse_header(record: &Record) -> String 
    {
        record.id().split('_').next().unwrap().to_string()
    }
}

pub fn run(
    input: String,
    output: Option<String>,
    include_sequence: bool,
    delim: Option<char>) -> Result<()> 
{
    let delim = match delim {
        Some(c) => c.to_string(),
        None => "\t".to_string()
    };
    let reader = fxread::initialize_reader(&input)?;
    let mut spinner = Spinner::new(Spinners::Dots12, "Mapping sgRNAs to Parent Genes".to_string());
    let table = Table::from_reader(reader);
    spinner.stop_and_persist("✔", format!("Mapped {} sgRNAs to {} Parent Genes", table.num_records(), table.num_genes()));
    match output {
        Some(f) => table.write_to_file(&f, &delim, include_sequence)?,
        None => table.write_to_stdout(&delim, include_sequence)
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use fxread::{FastxRead, Record, FastaReader, FastqReader};
    use super::Table;

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">AP2S1_ASJDAJSDAS\nACT\n>AP2S1_ASDKJASD\nACC\n>AP2S2_AOSDJIASJ\nACT\n";
        Box::new(FastaReader::new(sequence))
    }

    fn fastq_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b"@AP2S1_ASJDAJSDAS\nACT\n+\n123\n@AP2S1_ASDKJASD\nACC\n+\n123\n@AP2S2_AOSDJIASJ\nACT\n+\n123\n";
        Box::new(FastqReader::new(sequence))
    }

    #[test]
    fn table_fasta() {
        let reader = fasta_reader();
        let table = Table::from_reader(reader);
        assert_eq!(table.num_records(), 3);
        assert_eq!(table.num_genes(), 2)
    }

    #[test]
    fn table_fastq() {
        let reader = fastq_reader();
        let table = Table::from_reader(reader);
        assert_eq!(table.num_records(), 3);
        assert_eq!(table.num_genes(), 2)
    }
}
