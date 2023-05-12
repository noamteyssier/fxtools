use super::match_output_stream;
use anyhow::{bail, Result};
use fxread::{initialize_reader, FastxRead, Record};
use ndarray::{s, Array1, Array2, Axis};
use ndarray_stats::{EntropyExt, QuantileExt};
use spinoff::{Color, Spinner, Spinners, Streams};
use std::{io::Write, str::from_utf8};

/// Retrieves the sequence size of the first item in the reader
fn get_sequence_size(reader: &mut Box<dyn FastxRead<Item = Record>>) -> Result<usize> {
    if let Some(record) = reader.next() {
        Ok(record.seq().len())
    } else {
        bail!("Provided Reader is Empty")
    }
}

/// Assigns the provided byte to a nucleotide index
fn base_map(byte: u8) -> Option<usize> {
    match byte {
        b'A' => Some(0),
        b'C' => Some(1),
        b'G' => Some(2),
        b'T' => Some(3),
        _ => None,
    }
}

/// Increments the positional array for the provided indices
fn increment_positional_matrix(posmat: &mut Array2<f64>, pos_idx: usize, nuc_idx: Option<usize>) {
    if let Some(j) = nuc_idx {
        // increment the nucleotide index and at the position
        posmat[[pos_idx, j]] += 1.;
    } else {
        // increment each nucleotide index if an `N` is found (as it could be anything)
        posmat[[pos_idx, 0]] += 1.;
        posmat[[pos_idx, 1]] += 1.;
        posmat[[pos_idx, 2]] += 1.;
        posmat[[pos_idx, 3]] += 1.;
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
    num_samples: usize,
) -> Array2<f64> {
    let size = get_sequence_size(reader).unwrap();
    reader
        .take(num_samples)
        .fold(Array2::zeros((size, 4)), |mut posmat, record| {
            record
                .seq()
                .iter()
                .enumerate()
                .take(size)
                .map(|(idx, byte)| (idx, base_map(*byte)))
                .for_each(|(idx, jdx)| increment_positional_matrix(&mut posmat, idx, jdx));
            posmat
        })
}

/// Transforms a provided array via a Z-Score Calculation
fn zscore(array: &Array1<f64>) -> Array1<f64> {
    let mean = array.mean().unwrap_or(0.);
    let std = array.std(0.);
    array.map(|x| (x - mean) / std)
}

/// Calculates the positional entropy of the nucleotides for a provided set of records
fn calculate_positional_entropy(
    reader: &mut Box<dyn FastxRead<Item = Record>>,
    num_samples: usize,
) -> Array1<f64> {
    let pos_prob = normalize_counts(position_counts(reader, num_samples));
    pos_prob.map_axis(Axis(1), |axis| {
        axis.entropy().expect("Unexpected Negatives in Axis")
    })
}

/// Selects high entropy positions by applying a threshold on the zscore transformation of the
/// positional entropy vector
fn select_high_entropy_positions(
    positional_entropy: &Array1<f64>,
    zscore_threshold: f64,
) -> Array1<usize> {
    zscore(positional_entropy)
        .iter()
        .enumerate()
        .filter(|(_idx, x)| **x > zscore_threshold)
        .map(|(idx, _x)| idx)
        .collect()
}

fn find_longest_contiguous(array: &Array1<usize>) -> Array1<usize> {
    let (min, max) = array
        .iter()
        .enumerate()
        .fold((0, 0), |(mut min, mut max), (idx, x)| {
            if idx == 0 {
                return (0, 0);
            }
            if *x == array[idx - 1] + 1 {
                max = idx;
            } else {
                min = idx;
            }
            (min, max)
        });
    array.slice(s![min..=max]).to_owned()
}

/// Checks if the provided array of integers is contiguous
fn is_contiguous(array: &Array1<usize>) -> bool {
    array.iter().enumerate().all(|(idx, x)| {
        if idx == 0 {
            true
        } else {
            *x == array[idx - 1] + 1
        }
    })
}

/// Determines if high entropy positions are contiguous and attempts to calculate
/// the longest contiguous position if not
fn assign_contiguous(array: Array1<usize>) -> Result<Array1<usize>> {
    if is_contiguous(&array) {
        Ok(array)
    } else {
        let contiguous = find_longest_contiguous(&array);
        if contiguous.is_empty() {
            bail!("Cannot find a contiguous variable region!")
        }
        Ok(contiguous)
    }
}

/// Utility function to retrieve the minimum and maximum of a provided integer array
fn border(array: &Array1<usize>) -> Result<(usize, usize)> {
    if array.is_empty() {
        bail!("No entropies pass z-score threshold! Try lowering the threshold.")
    }
    let (min, max) = (*array.min()?, *array.max()?);
    if min == max {
        bail!("No entropies pass z-score threshold!")
    }
    Ok((min, max))
}

/// Writes the record as either fasta or fastq and applies the record sequence trimming to the
/// variable region
fn format_print(record: &Record, pos_min: usize, pos_max: usize) -> String {
    match record.qual() {
        Some(_) => {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(&record.seq()[pos_min..pos_max]).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(record.qual().unwrap()).expect("invalid utf8"),
            )
        }
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
fn write_to_output<W, I>(writer: &mut W, reader: I, pos_min: usize, pos_max: usize)
where
    W: Write,
    I: Iterator<Item = Record>,
{
    reader
        .map(|record| format_print(&record, pos_min, pos_max))
        .for_each(|x| write!(writer, "{}", x).expect("Error writing to file"));
}

/// Runs the variable region extraction
pub fn run(
    input: &str,
    output: Option<String>,
    num_samples: usize,
    zscore_threshold: f64,
) -> Result<()> {
    let spinner = Spinner::new_with_stream(
        Spinners::Dots12,
        format!("Calculating Entropy on {} Records", num_samples),
        Color::Green,
        Streams::Stderr,
    );

    // Calculate Positional Entropy && Select High Entropy Positions
    let mut reader = initialize_reader(input)?;
    let positional_entropy = calculate_positional_entropy(&mut reader, num_samples);
    let high_entropy_positions =
        select_high_entropy_positions(&positional_entropy, zscore_threshold);
    let contiguous_positions = assign_contiguous(high_entropy_positions)?;
    let (pos_min, pos_max) = border(&contiguous_positions)?;

    spinner.stop_with_message(
        &format!(
            "✔ Average Entropy: {:.3}\n✔ Minimum Entropy: {:.3}\n✔ Maximum Entropy: {:.3}\n✔ Bounds found: [{}, {}]",
            positional_entropy.mean().unwrap(),
            positional_entropy.min().unwrap(),
            positional_entropy.max().unwrap(),
            pos_min,
            pos_max));

    // Reinitialize reader and write to output
    let reader = initialize_reader(input)?;
    let mut writer = match_output_stream(output)?;
    write_to_output(&mut writer, reader, pos_min, pos_max);
    Ok(())
}

#[cfg(test)]
mod testing {
    use fxread::{FastaReader, FastxRead, Record};
    use ndarray::array;
    use ndarray_stats::EntropyExt;

    use crate::commands::extract::{
        calculate_positional_entropy, normalize_counts, select_high_entropy_positions,
    };

    use super::{base_map, border, find_longest_contiguous, is_contiguous, position_counts};

    #[test]
    fn test_base_map() {
        let bytes = b"ACGTN";
        assert_eq!(base_map(bytes[0]), Some(0));
        assert_eq!(base_map(bytes[1]), Some(1));
        assert_eq!(base_map(bytes[2]), Some(2));
        assert_eq!(base_map(bytes[3]), Some(3));
        assert_eq!(base_map(bytes[4]), None);
    }

    #[test]
    fn test_border() {
        let array = array![1, 2, 3, 4];
        assert_eq!(border(&array).unwrap(), (1, 4));
    }

    #[test]
    fn test_contiguous() {
        let array = array![1, 2, 3, 4];
        assert!(is_contiguous(&array));

        let array = array![1, 4, 3, 2];
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

    #[test]
    fn test_longest_contiguous() {
        let array = array![1, 3, 4, 5, 6];
        let cont = find_longest_contiguous(&array);
        assert_eq!(cont, array![3, 4, 5, 6]);
    }
}
