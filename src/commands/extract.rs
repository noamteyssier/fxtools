use std::{fs::File, io::{stdout, Write}};

use anyhow::Result;
use fxread::{initialize_reader, FastxRead, Record};
use ndarray::{Axis, Array2, Array1};
use ndarray_stats::{EntropyExt, QuantileExt};
use spinners::{Spinner, Spinners};
use std::str::from_utf8;

/// Retrieves the sequence size of the first item in the reader
fn get_sequence_size(
    reader: &mut Box<dyn FastxRead<Item = Record>>) -> usize
{
    reader.next().expect("empty reader").seq().len()
}

/// Assigns the provided byte to a nucleotide index
fn base_map(byte: &u8) -> Option<usize> {
    match byte {
        b'A' => Some(0),
        b'C' => Some(1),
        b'G' => Some(2),
        b'T' => Some(3),
        _ => None
    }
}

/// Increments the positional array for the provided indices
fn increment_positional_matrix(
    posmat: &mut Array2<f64>, 
    idx: usize, 
    jdx: Option<usize>)
{
    if let Some(j) = jdx {
        // increment the nucleotide index and at the position
        posmat[[idx, j]] += 1.;
    } else {
        // increment each nucleotide index if an `N` is found (as it could be anything)
        posmat[[idx, 0]] += 1.;
        posmat[[idx, 1]] += 1.;
        posmat[[idx, 2]] += 1.;
        posmat[[idx, 3]] += 1.;
    };
}

/// Normalizes the nucleotide counts across each row (i.e. sequence positional index)
fn normalize_counts(matrix: Array2<f64>) -> Array2<f64> {
    let (x, y) = matrix.dim();
    let sums = matrix.sum_axis(Axis(1));
    let norm = matrix / sums.broadcast((y, x)).expect("incompatible sizes").t();
    norm
}

/// Calculates the number of nucleotide occurences at each position in the sequences
fn position_counts(
    reader: &mut Box<dyn FastxRead<Item = Record>>,
    num_samples: usize) -> Array2<f64>
{
    let size = get_sequence_size(reader);
    reader
        .take(num_samples)
        .fold(
            Array2::zeros((size, 4)),
            |mut posmat, record| {
                record.seq().iter().enumerate()
                    .map(|(idx, byte)| (idx, base_map(byte)))
                    .for_each(|(idx, jdx)| increment_positional_matrix(&mut posmat, idx, jdx));
                posmat
            })
}

/// Transforms a provided array via a Z-Score Calculation
fn zscore(array: &Array1<f64>) -> Array1<f64>
{
    let mean = array.mean().unwrap_or(0.);
    let std = array.std(0.);
    array.map(|x| (x-mean)/std)
}

/// Calculates the positional entropy of the nucleotides for a provided set of records
fn calculate_positional_entropy(
    reader: &mut Box<dyn FastxRead<Item = Record>>,
    num_samples: usize) -> Array1<f64>
{
    let pos_prob = normalize_counts(position_counts(reader, num_samples));
    pos_prob.map_axis(Axis(1), |axis| axis.entropy().expect("Unexpected Negatives in Axis"))
}

/// Selects high entropy positions by applying a threshold on the zscore transformation of the
/// positional entropy vector
fn select_high_entropy_positions(
    positional_entropy: &Array1<f64>, 
    zscore_threshold: f64) -> Array1<usize>
{
    zscore(positional_entropy)
        .iter()
        .enumerate()
        .filter(|(_idx, x)| **x > zscore_threshold)
        .map(|(idx, _x)| idx)
        .collect()
}

/// Checks if the provided array of integers is contiguous
fn is_contiguous(
    array: &Array1<usize>) -> bool
{
    array
        .iter()
        .enumerate()
        .all(|(idx, x)| {
            if idx == 0 { true }
            else {
                *x == array[idx-1] + 1
            }
        })
}

/// Utility function to retrieve the minimum and maximum of a provided integer array
fn border(array: &Array1<usize>) -> Result<(usize, usize)>
{
    Ok((*array.min()?, *array.max()?))
}

/// Determines the output stream
fn assign_output(output: Option<String>) -> Result<Box<dyn Write>>
{
    match output {
        Some(s) => Ok(Box::new(File::create(s)?)),
        None => Ok(Box::new(stdout()))
    }
}

/// Writes the record as either fasta or fastq and applies the record sequence trimming to the
/// variable region
fn format_print(record: Record, pos_min: usize, pos_max: usize) -> String {
    match record.qual() {
        Some(_) => {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq()[pos_min..pos_max]).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(record.qual().unwrap()).expect("invalid utf8"),
                )
        },
        None => {
            format!(
                ">{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq()[pos_min..pos_max]).expect("invalid utf8")
                )
        }
    }
}

/// Writes results to output stream
fn write_to_output(
    reader: Box<dyn FastxRead<Item = Record>>, 
    output: Option<String>, 
    pos_min: usize, 
    pos_max: usize) -> Result<()>
{
    let mut writer = assign_output(output)?;
    reader
        .map(|record| format_print(record, pos_min, pos_max))
        .for_each(|x| write!(writer, "{}", x).expect("Error writing to file"));
    Ok(())
}

/// Runs the variable region extraction
pub fn run(
    input: String, 
    output: Option<String>, 
    num_samples: usize,
    zscore_threshold: f64) -> Result<()>
{
    // Calculate Positional Entropy && Select High Entropy Positions
    let mut reader = initialize_reader(&input)?;

    let mut spinner = Spinner::new(Spinners::Dots12, format!("Calculating Entropy on {} Records", num_samples));
    let positional_entropy = calculate_positional_entropy(&mut reader, num_samples);
    let high_entropy_positions = select_high_entropy_positions(&positional_entropy, zscore_threshold);
    assert!(is_contiguous(&high_entropy_positions), "High Entropy Positions must be contiguous -- try raising the zscore threshold");
    let (pos_min, pos_max) = border(&high_entropy_positions)?;
    spinner.stop_with_message(
        format!(
            "✔ Average Entropy: {:.3}\n✔ Minimum Entropy: {:.3}\n✔ Maximum Entropy: {:.3}\n✔ Bounds found: [{}, {}]",
            positional_entropy.mean().unwrap(),
            positional_entropy.min().unwrap(),
            positional_entropy.max().unwrap(),
            pos_min,
            pos_max));

    // Reinitialize reader and write to output
    let reader = initialize_reader(&input)?;
    write_to_output(reader, output, pos_min, pos_max)?;

    Ok(())
}

#[cfg(test)]
mod testing {
    use fxread::{FastxRead, FastaReader, Record};
    use ndarray::array;
    use ndarray_stats::EntropyExt;

    use crate::commands::extract::{normalize_counts, calculate_positional_entropy, select_high_entropy_positions};

    use super::{base_map, border, is_contiguous, position_counts};

    #[test]
    fn test_base_map() {
        let bytes = b"ACGTN";
        assert_eq!(base_map(&bytes[0]), Some(0));
        assert_eq!(base_map(&bytes[1]), Some(1));
        assert_eq!(base_map(&bytes[2]), Some(2));
        assert_eq!(base_map(&bytes[3]), Some(3));
        assert_eq!(base_map(&bytes[4]), None);
    }

    #[test]
    fn test_border() {
        let array = array![1,2,3,4];
        assert_eq!(border(&array).unwrap(), (1, 4));
    }

    #[test]
    fn test_contiguous() {
        let array = array![1,2,3,4];
        assert!(is_contiguous(&array));

        let array = array![1,4,3,2];
        assert!(!is_contiguous(&array));
    }

    #[test]
    fn test_position_counts() {
        let fasta: &'static [u8] = b">seq.0\nACGT\n>seq.1\nACGT\n>seq.2\nACGT\n";
        let mut reader: Box<dyn FastxRead<Item = Record>> = Box::new(FastaReader::new(fasta));
        let posmat = position_counts(&mut reader, 3);

        // position 0; A
        assert_eq!(posmat[[0, 0]], 2.);

        // position 0; C
        assert_eq!(posmat[[0, 1]], 0.);

        // position 4; T
        assert_eq!(posmat[[3, 3]], 2.);
    }

    #[test]
    fn test_position_frequency() {
        let fasta: &'static [u8] = b">seq.0\nACGT\n>seq.1\nACGT\n>seq.2\nACGT\n";
        let mut reader: Box<dyn FastxRead<Item = Record>> = Box::new(FastaReader::new(fasta));
        let posmat = position_counts(&mut reader, 3);
        let pos_prob = normalize_counts(posmat);

        // position 0; A
        assert_eq!(pos_prob[[0, 0]], 1.);

        // position 0; C
        assert_eq!(pos_prob[[0, 1]], 0.);

        // position 4; T
        assert_eq!(pos_prob[[3, 3]], 1.);
    }

    #[test]
    fn test_positional_entropy_none() {
        let fasta: &'static [u8] = b">seq.0\nACGT\n>seq.1\nACGT\n>seq.2\nACGT\n";
        let mut reader: Box<dyn FastxRead<Item = Record>> = Box::new(FastaReader::new(fasta));
        let entropy = calculate_positional_entropy(&mut reader, 3);
        assert!(entropy.iter().all(|x| *x == 0.));
    }

    #[test]
    fn test_positional_entropy_high() {
        let fasta: &'static [u8] = b">seq.0\nACGT\n>seq.1\nTCGA\n>seq.2\nGATC\n";
        let mut reader: Box<dyn FastxRead<Item = Record>> = Box::new(FastaReader::new(fasta));
        let entropy = calculate_positional_entropy(&mut reader, 3);
        let value = array![0.5, 0.5].entropy().unwrap();
        assert!(entropy.iter().all(|x| *x == value));
    }

    #[test]
    fn test_high_entropy_positions() {
        let fasta: &'static [u8] = b">seq.0\nACGT\n>seq.1\nACGT\n>seq.2\nAGCT\n";
        let mut reader: Box<dyn FastxRead<Item = Record>> = Box::new(FastaReader::new(fasta));
        let entropy = calculate_positional_entropy(&mut reader, 3);
        let positions = select_high_entropy_positions(&entropy, 0.5);
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0], 1);
        assert_eq!(positions[1], 2);
    }
}
