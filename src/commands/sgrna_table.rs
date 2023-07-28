use anyhow::Result;
use fxread::{FastxRead, Record, initialize_reader, initialize_stdin_reader};
use spinoff::{Color, Spinner, Spinners, Streams};
use std::{
    collections::HashMap,
    fs::File,
    io::{stdout, Write, stdin},
    str::from_utf8,
};

/// Creates a mapping of gene names to `sgRNA` names
struct Table {
    map: HashMap<Vec<u8>, Vec<Record>>,
}
impl Table {
    /// creates a table from a [`FastxRead`] reader.
    pub fn from_reader(reader: Box<dyn FastxRead<Item = Record>>) -> Self {
        let map = Self::build(reader);
        Self { map }
    }

    /// Returns the number of genes found
    pub fn num_genes(&self) -> usize {
        self.map.len()
    }

    /// Returns the number of records mapped
    pub fn num_records(&self) -> usize {
        self.map.values().flatten().count()
    }

    /// Matches the output stream from output
    pub fn match_stream(output: Option<String>) -> Result<Box<dyn Write>> {
        match output {
            Some(path) => Ok(Box::new(File::create(path)?)),
            None => Ok(Box::new(stdout())),
        }
    }

    /// Write table to output stream
    pub fn write_result(
        &self,
        writer: &mut Box<dyn Write>,
        delim: u8,
        include_sequence: bool,
        order: &str,
    ) {
        self.map.iter().for_each(|(k, v)| {
            v.iter().for_each(|record| {
                writeln!(
                    writer,
                    "{}",
                    from_utf8(&Self::prepare_result(
                        k,
                        delim,
                        record,
                        include_sequence,
                        order
                    ))
                    .unwrap()
                )
                .expect("Writing Error");
            })
        });
    }

    /// Maps an ordering character to its respective string token
    fn map_token<'a>(
        c: char,
        gene: &'a [u8],
        record: &'a Record,
        include_sequence: bool,
    ) -> Option<&'a [u8]> {
        match c {
            'g' | 'G' => Some(gene),
            'h' | 'H' => Some(record.id()),
            's' | 'S' => {
                if include_sequence {
                    Some(record.seq())
                } else {
                    None
                }
            }
            _ => panic!("Unexpected character in GSH token: {}", c),
        }
    }

    /// Builds the string for the row and handles delimiter addition
    fn build_row(row: &mut Vec<u8>, idx: usize, token: Option<&[u8]>, delim: u8) -> Vec<u8> {
        if let Some(t) = token {
            if idx > 0 {
                row.push(delim);
            }
            row.extend_from_slice(t);
        }
        row.clone()
    }

    /// Properly formats the string for output
    fn prepare_result(
        gene: &[u8],
        delim: u8,
        record: &Record,
        include_sequence: bool,
        order: &str,
    ) -> Vec<u8> {
        order
            .chars()
            .map(|c| Self::map_token(c, gene, record, include_sequence))
            .enumerate()
            .fold(Vec::new(), |mut row, (idx, token)| {
                Self::build_row(&mut row, idx, token, delim)
            })
    }

    /// main build iterator
    fn build(reader: Box<dyn FastxRead<Item = Record>>) -> HashMap<Vec<u8>, Vec<Record>> {
        reader.fold(HashMap::new(), |mut table, record| {
            let gene = Self::parse_header(&record);
            table.entry(gene).or_insert(Vec::new()).push(record);
            table
        })
    }

    /// parses the gene name from the record header
    fn parse_header(record: &Record) -> Vec<u8> {
        match record.id().split(|b| *b == b'_').next() {
            Some(split) => split.to_owned(),
            None => record.id().to_owned(),
        }
    }
}

/// Validate that all characters in the order string are expected and known
fn validate_characters(order: &str) -> bool {
    order
        .chars()
        .map(|c| matches!(c, 'G' | 'S' | 'H' | 'g' | 's' | 'h'))
        .all(|c| c)
}

/// Validates that the order string is within the expected bounds and contains
/// expected characters
fn validate_order(order: &str) {
    assert!(
        validate_characters(order),
        "Unrecognized characters in reorder: {}",
        order
    );
    assert!(
        order.len() <= 3,
        "Ordering lengh must be less than 3: {}",
        order
    );
}

pub fn run(
    input: Option<String>,
    output: Option<String>,
    include_sequence: bool,
    delim: Option<char>,
    reorder: Option<String>,
) -> Result<()> {
    let delim = match delim {
        Some(c) => c as u8,
        None => b'\t',
    };

    let order = match reorder {
        Some(o) => o,
        None => String::from("ghs"),
    };

    validate_order(&order);

    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;

    let spinner = Spinner::new_with_stream(
        Spinners::Dots12,
        "Mapping sgRNAs to Parent Genes".to_string(),
        Color::Green,
        Streams::Stderr,
    );
    let table = Table::from_reader(reader);
    spinner.stop_and_persist(
        "✔",
        &format!(
            "Mapped {} sgRNAs to {} Parent Genes",
            table.num_records(),
            table.num_genes()
        ),
    );
    let mut writer = Table::match_stream(output)?;
    table.write_result(&mut writer, delim, include_sequence, &order);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Table;
    use fxread::{FastaReader, FastqReader, FastxRead, Record};

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] =
            b">AP2S1_ASJDAJSDAS\nACT\n>AP2S1_ASDKJASD\nACC\n>AP2S2_AOSDJIASJ\nACT\n";
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
