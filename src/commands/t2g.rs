use std::io::stdin;
use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};
use super::match_output_stream;

fn get_line(record: &Record, symbol: bool, dot_version: bool) -> String {

    // select the id
    let id = record.id_str();

    // split the id into attributes
    let mut attributes = id.split_whitespace();

    // get the transcript id
    let transcript_id = attributes.next().expect("No ensembl id");

    // get the gene id
    let gene_id = if dot_version {
        attributes.next().expect("No gene id")
            .replace("gene_id:", "")
    } else {
        attributes.next().expect("No gene id")
            .split(".")
            .nth(0)
            .expect("No gene id")
            .replace("gene_id:", "")
    };

    // get the gene symbol if requested
    if symbol {
        let gene_symbol = attributes.next().expect("No gene symbol")
            .replace("gene_name:", "");

        // if the gene symbol is empty, just return the transcript and gene id
        if gene_symbol.is_empty() {
            format!("{}\t{}\n", transcript_id, gene_id)
        } else {
            format!("{}\t{}\n", transcript_id, gene_symbol)
        }

    // otherwise just return the transcript and gene id
    } else {
        format!("{}\t{}\n", transcript_id, gene_id)
    }
}


pub fn run(
    input: Option<String>,
    output: Option<String>,
    symbol: bool,
    dot_version: bool,
    num_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, num_threads, compression_level)?;
    reader
        .map(|x| get_line(&x, symbol, dot_version))
        .for_each(|x| {
            write!(writer, "{x}").expect("Error Writing to File");
        });
    Ok(())
}

#[cfg(test)]
mod testing {
    use fxread::{FastxRead, Record, FastaReader};
    use super::*;

    
    fn fasta_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = 
            b">ENST00000003583.12 gene_id:ENSG00000001460.18 gene_name:STPG1 transcript_name:STPG1-201 chr:1 start:24357005 end:24413725 strand:-\nACTACTACT";
        Box::new(FastaReader::new(sequence))
    }

    fn fasta_missing_symbol() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = 
            b">ENST00000003583.12 gene_id:ENSG00000001460.18 gene_name: transcript_name:STPG1-201 chr:1 start:24357005 end:24413725 strand:-\nACTACTACT";
        Box::new(FastaReader::new(sequence))
    }

    #[test]
    fn run_fasta_nodot_gene_id() {
        let mut reader = fasta_reader();
        let record = reader.next().unwrap();
        let line = get_line(&record, false, false);
        assert_eq!(line, "ENST00000003583.12\tENSG00000001460\n");
    }

    #[test]
    fn run_fasta_nodot_gene_symbol() {
        let mut reader = fasta_reader();
        let record = reader.next().unwrap();
        let line = get_line(&record, true, false);
        assert_eq!(line, "ENST00000003583.12\tSTPG1\n");
    }

    #[test]
    fn run_fasta_dot_gene_id() {
        let mut reader = fasta_reader();
        let record = reader.next().unwrap();
        let line = get_line(&record, false, true);
        assert_eq!(line, "ENST00000003583.12\tENSG00000001460.18\n");
    }

    #[test]
    fn run_fasta_nodot_missing_gene_name() {
        let mut reader = fasta_missing_symbol();
        let record = reader.next().unwrap();
        let line = get_line(&record, true, false);
        assert_eq!(line, "ENST00000003583.12\tENSG00000001460\n");
    }

    #[test]
    fn run_fasta_dot_missing_gene_name() {
        let mut reader = fasta_missing_symbol();
        let record = reader.next().unwrap();
        let line = get_line(&record, true, true);
        assert_eq!(line, "ENST00000003583.12\tENSG00000001460.18\n");
    }

}
