use super::match_output_stream;
use anyhow::{bail, Result};
use fxread::initialize_reader;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

const LEXICON: [u8; 4] = [b'A', b'C', b'G', b'T'];

#[derive(Debug, Serialize)]
struct MultiplexLog {
    num_inputs: usize,
    barcode_size: usize,
    barcodes: HashMap<String, String>,
}

fn minimum_barcode_size(n_inputs: usize) -> usize {
    let lex_size = LEXICON.len();
    for i in 1.. {
        if lex_size.pow(i) >= n_inputs {
            return i as usize;
        }
    }
    unreachable!()
}

fn generate_barcodes(
    n_inputs: usize,
    barcode_size: usize,
    seed: Option<u64>,
    timeout: u64,
) -> Result<Vec<Vec<u8>>> {
    let mut barcodes = Vec::with_capacity(n_inputs);
    let mut rng = ChaCha8Rng::seed_from_u64(seed.unwrap_or_default());
    let mut num_trials = 0;
    loop {
        let sample_barcode = (0..barcode_size)
            .map(|_| rng.gen_range(0..LEXICON.len()))
            .map(|i| LEXICON[i])
            .collect::<Vec<_>>();
        if !barcodes.contains(&sample_barcode) {
            barcodes.push(sample_barcode);
            if barcodes.len() == n_inputs {
                break;
            }
        } else {
            if num_trials > timeout {
                break;
            }
            num_trials += 1;
        }
    }
    if barcodes.len() < n_inputs {
        panic!("Could not generate enough unique barcodes - try setting another seed or increasing the barcode size")
    } else {
        Ok(barcodes)
    }
}

fn verify_unique_inputs(inputs: Vec<String>) -> Result<()> {
    let mut unique_inputs = inputs.clone();
    unique_inputs.sort();
    unique_inputs.dedup();
    if unique_inputs.len() != inputs.len() {
        bail!("Input files must be unique")
    } else {
        Ok(())
    }
}

fn write_whitelist(
    input_whitelist: Option<String>,
    barcodes: &[Vec<u8>],
    output_filename: &str,
) -> Result<()> {
    if let Some(input_filename) = input_whitelist {
        eprintln!("Writing updated whitelist to {}", output_filename);
        if input_filename == output_filename {
            bail!("Input and output whitelist files must be different")
        }
        let mut writer = File::create(output_filename).map(BufWriter::new)?;
        for barcode in barcodes {
            let reader = File::open(&input_filename).map(BufReader::new)?;
            for line in reader.lines() {
                let line = line?;
                let mut bytes = line.into_bytes();
                bytes.splice(0..0, barcode.clone());
                writer.write_all(&bytes)?;
                writer.write_all(b"\n")?;
            }
        }
    }
    Ok(())
}

/// Runs the `multiplex` command.
#[allow(clippy::too_many_arguments)]
pub fn run(
    inputs: Vec<String>,
    output: Option<String>,
    whitelist: Option<String>,
    output_whitelist: String,
    log: String,
    barcode_size: Option<usize>,
    seed: Option<u64>,
    timeout: u64,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    // Verify that inputs are unique
    verify_unique_inputs(inputs.clone())?;

    // Initialize output stream
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;

    // Generate barcodes
    let minimum_barcode_size = minimum_barcode_size(inputs.len());
    let barcode_size = if let Some(b) = barcode_size {
        b.max(minimum_barcode_size)
    } else {
        minimum_barcode_size
    };
    let sample_barcodes = generate_barcodes(inputs.len(), barcode_size, seed, timeout)?;

    // Append barcodes to reads and write to output
    let mut barcode_map = HashMap::new();
    for (sample_idx, input) in inputs.iter().enumerate() {
        let sample_barcode = &sample_barcodes[sample_idx];
        barcode_map.insert(
            input.clone(),
            String::from_utf8(sample_barcode.clone()).unwrap(),
        );
        let reader = initialize_reader(input)?;
        for record in reader {
            let mut record = record;
            record.insert_seq_left(&sample_barcodes[sample_idx])?;
            write!(writer, "{}", record.as_str())?;
        }
    }

    // Append barcodes to whitelist and write to whitelist output if provided
    write_whitelist(whitelist, &sample_barcodes, &output_whitelist)?;

    // Generate log
    let output_log = MultiplexLog {
        num_inputs: inputs.len(),
        barcode_size,
        barcodes: barcode_map,
    };

    // Write log
    eprintln!("Writing log to {}", log);
    let mut log_writer = match_output_stream(Some(log), compression_threads, compression_level)?;
    write!(log_writer, "{}", serde_json::to_string(&output_log)?)?;
    Ok(())
}
