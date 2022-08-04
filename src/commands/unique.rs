use std::collections::HashMap;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use spinoff::{Spinner, Spinners, Color, Streams};
use fxread::{initialize_reader, FastxRead, Record};
use std::str::from_utf8;

type UniqMap = HashMap<Vec<u8>, Record>;
type NullMap = HashMap<Vec<u8>, Vec<Record>>;

struct Unique {
    map: UniqMap,
    null: NullMap
}
impl Unique {
    
    /// Initializes the Unique Set
    pub fn from_reader(reader: Box<dyn FastxRead<Item = Record>>) -> Self 
    {
        let (map, null) = Self::build(reader);
        Self { map, null }
    }

    /// Return all records with unique sequences
    pub fn passing_records(&self) -> impl Iterator<Item = &Record> 
    {
        self.map.values()
    }

    /// Return all records with non-unique sequences
    pub fn null_records(&self) -> impl Iterator<Item = &Record> 
    {
        self.null.values().flatten()
    }

    /// Return number of unique records
    pub fn num_passing(&self) -> usize
    {
        self.map.len()
    }

    /// Return number of null records
    pub fn num_null_records(&self) -> usize 
    {
        self.null.values().flatten().count()
    }

    /// Return number of null sequences
    pub fn num_null_sequences(&self) -> usize
    {
        self.null.len()
    }

    /// Reads in the records and performs the unique matching
    fn build(
        reader: Box<dyn FastxRead<Item = Record>>) -> (UniqMap, NullMap) 
    {
        reader
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut map, mut null), x| {

                    // continues if not already nulled
                    if !Self::in_null(&mut null, &x) {

                        // nullify if in map already
                        if Self::in_map(&mut map, &x) {
                            Self::nullify_existing(&mut null, &mut map, x)
                        }

                        // insert to map
                        else {
                            Self::insert_to_map(&mut map, x)
                        }
                    }

                    // nullify record with duplicate sequence
                    else {
                        Self::insert_to_null(&mut null, x)
                    }

                    (map, null)
                })
    }

    /// checks whether the record's sequence exists in the current
    /// positive set
    fn in_map(
            map: &mut UniqMap, 
            record: &Record) -> bool 
    {
        map.contains_key(record.seq())
    }

    /// Checks whether the records sequence exists in the current
    /// null set 
    fn in_null(
            null: &mut NullMap, 
            record: &Record) -> bool 
    {
        null.contains_key(record.seq())
    }

    /// Inserts a null sequence to the set and removes it from the map
    fn nullify_existing(
            null: &mut NullMap, 
            map: &mut UniqMap,
            record: Record) 
    {
        let duplicate = map.remove(record.seq()).expect("unexpected empty value");
        Self::insert_to_null(null, duplicate);
        Self::insert_to_null(null, record)

    }

    /// Inserts a sequence to null
    fn insert_to_null(
            null: &mut NullMap,
            record: Record) 
    {
        null
            .entry(record.seq().to_owned())
                .or_insert(Vec::new())
            .push(record);
    }

    /// Inserts a sequence to the map
    fn insert_to_map(
            map: &mut UniqMap,
            record: Record) 
    {
        map.insert(record.seq().to_owned(), record);
    }
}

fn format_print(record: &Record) -> String {
    match record.qual() {
        Some(_) => {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(record.seq()).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(record.qual().unwrap()).expect("invalid utf8"),
                )
        },
        None => {
            format!(
                ">{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(record.seq()).expect("invalid utf8")
                )
        }
    }
}

/// Writes the output fasta to stdout
fn write_to_stdout(unique: &Unique)
{
    unique
        .passing_records()
        .for_each(|x| 
            print!("{}", format_print(x)));
}

/// Writes the output fasta
fn write_output(
        output: &str, 
        unique: &Unique) -> Result<()>
{
    unique
        .passing_records()
        .fold(
            File::create(output).expect("Could not Create Output File"),
            |mut f, x| {
                write!(f, "{}", format_print(x)).expect("Error writing to output");
                f
            });
    Ok(())
}

/// Writes the null fasta
fn write_null(
        output: &str, 
        unique: &Unique) -> Result<()>
{
    unique
        .null_records()
        .fold(
            File::create(output).expect("Could not Create Null File"),
            |mut f, x| {
                write!(f, "{}", format_print(x)).expect("Error writing to null");
                f
            });
    Ok(())
}

pub fn run(
        path: &str, 
        output: Option<String>, 
        null: Option<String>) -> Result<()> 
{
    let reader = initialize_reader(path)?;

    let spinner = Spinner::new_with_stream(
        Spinners::Dots12, 
        "Determining Unique Records".to_string(),
        Color::Green,
        Streams::Stderr);
    let unique = Unique::from_reader(reader);
    spinner.stop_and_persist(
        "✔", 
        &format!(
            "Found {} unique records, {} duplicate sequences with {} records affected", 
            unique.num_passing(), 
            unique.num_null_sequences(), 
            unique.num_null_records()));
    
    match output {
        Some(file_handle) => write_output(&file_handle, &unique)?,
        None => write_to_stdout(&unique)
    };
    
    if let Some(file_handle) = null { write_null(&file_handle, &unique)? }
    
    Ok(())
}

#[cfg(test)]
mod test {
    use fxread::{FastxRead, Record, FastaReader, FastqReader};
    use super::Unique;

    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nACT\n>seq.1\nACC\n>seq.2\nACT\n";
        Box::new(FastaReader::new(sequence))
    }

    fn fastq_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b"@seq.0\nACT\n+\n123\n@seq.1\nACC\n+\n123\n@seq.2\nACT\n+\n123\n";
        Box::new(FastqReader::new(sequence))
    }

    #[test]
    fn unique_fasta() {
        let reader = fasta_reader();
        let unique = Unique::from_reader(reader);
        assert_eq!(unique.num_null_records(), 2);
        assert_eq!(unique.num_null_sequences(), 1);
        assert_eq!(unique.num_passing(), 1);
    }

    #[test]
    fn unique_fastq() {
        let reader = fastq_reader();
        let unique = Unique::from_reader(reader);
        assert_eq!(unique.num_null_records(), 2);
        assert_eq!(unique.num_null_sequences(), 1);
        assert_eq!(unique.num_passing(), 1);
    }

}
