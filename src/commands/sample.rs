use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::io::{stdin, Write};

use super::match_output_stream;

/// Runs the `sample` command.
pub fn run(
    input: Option<String>,
    output: Option<String>,
    freq: f64,
    seed: Option<u64>,
    quiet: bool,
    compression_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut rng = match seed {
        Some(seed) => ChaChaRng::seed_from_u64(seed),
        None => ChaChaRng::from_entropy(),
    };
    let mut writer = match_output_stream(output, compression_threads, compression_level)?;
    let mut num_records = 0;
    for record in reader {
        if rng.gen_bool(freq) {
            write!(writer, "{}", record.as_str())?;
            num_records += 1;
        }
    }

    if !quiet {
        eprintln!("{} records sampled", num_records);
    }
    Ok(())
}
