use super::match_output_stream;
use anyhow::Result;
use disambiseq::Disambibyte;
use fxread::{initialize_reader, initialize_stdin_reader, Record};
use hashbrown::HashMap;
use std::io::{stdin, Write};

type Sequence = Vec<u8>;
type Header = Vec<u8>;

fn seq_to_header<I>(it: I) -> HashMap<Sequence, Header>
where
    I: Iterator<Item = Record>,
{
    let mut map = HashMap::new();
    for record in it {
        map.insert(record.seq().to_vec(), record.id().to_vec());
    }
    map
}

fn header_counts<'a, I>(it: I) -> HashMap<Header, usize>
where
    I: Iterator<Item = &'a Header>,
{
    let mut map = HashMap::new();
    for record in it {
        let header = record.to_owned();
        *map.entry(header).or_insert(0) += 1;
    }
    map
}

pub fn run(
    input: Option<String>,
    output: Option<String>,
    include_parents: bool,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    // Initialize reader
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;

    // Create the parent sequence to header name mapping table
    let seq_map = seq_to_header(reader);

    // Create a table of header counts to be populated in the unambiguous loop
    let mut header_counts = header_counts(seq_map.values());

    // Initialize the parent sequence slice and build all unambiguous one-off mutants
    let key_slice = seq_map.keys().cloned().collect::<Vec<Sequence>>();
    let dq = Disambibyte::from_slice(key_slice.as_slice());

    // Match the output stream
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;

    if include_parents {
        // Iterate through parent sequences
        for parent in seq_map.keys() {
            let header = seq_map.get(parent).unwrap();
            let header_str = std::str::from_utf8(header)?;
            let parent_str = std::str::from_utf8(parent)?;
            writeln!(writer, ">{header_str}\n{parent_str}")?;
        }
    }

    // Iterate through unambiguous mutants
    for un in dq.unambiguous() {
        let (mutant, parent) = un;
        let header = seq_map.get(parent.sequence()).unwrap();

        // Retrieve the header count for the parent sequence
        let count = header_counts.get(header).unwrap();

        let header_str = std::str::from_utf8(header)?;
        let mutant_str = std::str::from_utf8(mutant.sequence())?;
        writeln!(writer, ">{header_str}_{count}\n{mutant_str}")?;

        // Increment the header count for the parent sequence
        *header_counts.get_mut(header).unwrap() += 1;
    }
    Ok(())
}
