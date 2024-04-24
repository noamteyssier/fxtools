use super::match_output_stream;
use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};
use std::io::stdin;

pub fn clip(record: Record, start: Option<usize>, end: Option<usize>) -> Result<Record> {
    let mut record = record;
    if let Some(start) = start {
        record.trim_left(start)?;
    }
    if let Some(end) = end {
        record.trim_right(end)?;
    }
    Ok(record)
}

pub fn clip_to_range(record: Record, start: Option<usize>, end: Option<usize>) -> Result<Record> {
    let mut record = record;

    if start.is_some() && end.is_some() {
        let start = start.unwrap();
        let end = end.unwrap();
        let left_idx = record.seq().len() - start - end;
        record.trim_left(start)?;
        record.trim_right(left_idx)?;
        Ok(record)
    } else if let Some(start) = start {
        record.trim_left(start)?;
        Ok(record)
    } else if let Some(end) = end {
        let left_idx = record.seq().len() - end;
        record.trim_right(left_idx)?;
        Ok(record)
    } else {
        Ok(record)
    }
}

fn parse_range(range: String) -> Result<(Option<usize>, Option<usize>)> {
    if let Some(end) = range.strip_prefix("..") {
        let end = end.parse::<usize>().unwrap();
        Ok((None, Some(end)))
    } else if let Some(start) = range.strip_suffix("..") {
        let start = start.parse::<usize>().unwrap();
        Ok((Some(start), None))
    } else {
        let mut range = range.split("..");
        let start = range.next().unwrap().parse::<usize>()?;
        let end = range.next().unwrap().parse::<usize>()?;
        Ok((Some(start), Some(end)))
    }
}

/// Runs the `clip` command.
pub fn run(
    input: Option<String>,
    output: Option<String>,
    start: Option<usize>,
    end: Option<usize>,
    range: Option<String>,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;
    if let Some(range) = range {
        let (start, end) = parse_range(range.clone())?;
        for record in reader {
            let record = clip_to_range(record, start, end)?;
            write!(writer, "{}", record.as_str())?;
        }
    } else {
        for record in reader {
            let record = clip(record, start, end)?;
            write!(writer, "{}", record.as_str())?;
        }
    }
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
    fn test_parse_range() {
        let range = "1..2".to_string();
        let (start, end) = parse_range(range).unwrap();
        assert_eq!(start, Some(1));
        assert_eq!(end, Some(2));

        let range = "1..".to_string();
        let (start, end) = parse_range(range).unwrap();
        assert_eq!(start, Some(1));
        assert_eq!(end, None);

        let range = "..2".to_string();
        let (start, end) = parse_range(range).unwrap();
        assert_eq!(start, None);
        assert_eq!(end, Some(2));
    }

    #[test]
    fn fasta_clip_left() {
        let mut reader = fasta_reader();
        let start = Some(1);
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip(record, start, None).unwrap();
        assert_eq!(record.seq(), &seq[1..]);
    }

    #[test]
    fn fasta_clip_right() {
        let mut reader = fasta_reader();
        let end = Some(1);
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip(record, None, end).unwrap();
        assert_eq!(record.seq(), &seq[..seq.len() - 1]);
    }

    #[test]
    fn fasta_no_clip() {
        let mut reader = fasta_reader();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip(record, None, None).unwrap();
        assert_eq!(record.seq(), &seq[..]);
    }

    #[test]
    fn fasta_clip_range_both() {
        let mut reader = fasta_reader();
        let range = Some("1..2".to_string());
        let (start, end) = parse_range(range.clone().unwrap()).unwrap();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip_to_range(record, start, end).unwrap();
        assert_eq!(record.seq(), &seq[1..=2]);
    }

    #[test]
    fn fasta_clip_range_left() {
        let mut reader = fasta_reader();
        let range = Some("1..".to_string());
        let (start, end) = parse_range(range.clone().unwrap()).unwrap();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip_to_range(record, start, end).unwrap();
        assert_eq!(record.seq(), &seq[1..]);
    }

    #[test]
    fn fasta_clip_range_right() {
        let mut reader = fasta_reader();
        let range = Some("..2".to_string());
        let (start, end) = parse_range(range.clone().unwrap()).unwrap();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip_to_range(record, start, end).unwrap();
        assert_eq!(record.seq(), &seq[..2]);
    }

    #[test]
    fn fastq_clip_left() {
        let mut reader = fastq_reader();
        let start = Some(1);
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip(record, start, None).unwrap();
        assert_eq!(record.seq(), &seq[1..]);
    }

    #[test]
    fn fastq_clip_right() {
        let mut reader = fastq_reader();
        let end = Some(1);
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip(record, None, end).unwrap();
        assert_eq!(record.seq(), &seq[..seq.len() - 1]);
    }

    #[test]
    fn fastq_no_clip() {
        let mut reader = fastq_reader();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip(record, None, None).unwrap();
        assert_eq!(record.seq(), &seq[..]);
    }

    #[test]
    fn fastq_clip_range_both() {
        let mut reader = fastq_reader();
        let range = Some("1..2".to_string());
        let (start, end) = parse_range(range.clone().unwrap()).unwrap();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip_to_range(record, start, end).unwrap();
        assert_eq!(record.seq(), &seq[1..=2]);
    }

    #[test]
    fn fastq_clip_range_left() {
        let mut reader = fastq_reader();
        let range = Some("1..".to_string());
        let (start, end) = parse_range(range.clone().unwrap()).unwrap();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip_to_range(record, start, end).unwrap();
        assert_eq!(record.seq(), &seq[1..]);
    }

    #[test]
    fn fastq_clip_range_right() {
        let mut reader = fastq_reader();
        let range = Some("..2".to_string());
        let (start, end) = parse_range(range.clone().unwrap()).unwrap();
        let record = reader.next().unwrap();
        let seq = record.seq().to_vec();
        let record = clip_to_range(record, start, end).unwrap();
        assert_eq!(record.seq(), &seq[..2]);
    }
}
