use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Compression threads to use for output files if applicable
    #[clap(global = true, short = 'j', long)]
    pub compression_threads: Option<usize>,

    /// Compression level to use for output files if applicable
    #[clap(global = true, short = 'Z', long)]
    pub compression_level: Option<usize>,
}

#[derive(Subcommand)]
enum Commands {
    /// Counts the number of records in a Fastx file
    Count {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Count
        input: Option<String>,
    },

    /// Filters same length sequences to their variable region. Useful in CRISPRi/a libraries where
    /// the variable region is prefixed and suffixed by some constant region
    ExtractVariable {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to to extract variable region
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser, default_value = "5000")]
        /// Number of samples to calculate positional entropy on
        num_samples: usize,

        #[clap(short, long, value_parser, default_value = "1.0")]
        /// Number of samples to calculate positional entropy on
        zscore_threshold: f64,
    },

    /// Filters a fastx file by searching for whether they follow a regex pattern on the sequence
    Filter {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to fix
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Regex pattern to search for
        pattern: String,

        #[clap(short = 'v', long, value_parser, default_value = "false")]
        /// Whether to invert the filter
        invert: bool,

        #[clap(short = 'H', long, value_parser, default_value = "false")]
        /// Whether to search for the pattern in the header
        header: bool,
    },

    /// Fix a fastx file by replacing invalid characters with N
    Fix {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to fix
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,
    },

    /// Creates the Reverse complement for a provided fastx
    Reverse {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Convert to Upper
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,
    },

    /// Samples a fastx file by a frequency
    Sample {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to sample
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser, default_value = "0.5")]
        /// Frequency to sample by
        frequency: f64,

        #[clap(short, long, value_parser)]
        /// Seed to use for sampling
        seed: Option<u64>,

        #[clap(short, long, value_parser, default_value = "false")]
        /// Don't write number of records sampled to stderr
        quiet: bool,
    },

    /// Creates a mapping of sgRNAs to their parent gene
    SgrnaTable {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Generate table
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write table to [default: stdout]
        output: Option<String>,

        #[clap(short = 's', long, action)]
        /// Whether to include the sequence in the output table [default: false]
        include_sequence: bool,

        #[clap(short, long, value_parser)]
        /// Specify ordering of columns as 3 value string ([Hh]eader, [Ss]equence, [Gg]ene).
        /// [default: ghs]
        reorder: Option<String>,

        #[clap(short, long, value_parser)]
        /// Optional choice of output delimiter [default: '\t']
        delim: Option<char>,
    },

    /// Sorts a fastx file by sequence
    Sort {
        #[clap(short = 'i', long, value_parser)]
        /// Input FASTA/Q to sort
        r1: Option<String>,

        #[clap(short = 'I', long, value_parser)]
        /// Optional choice of R2 to sort by
        r2: Option<String>,

        #[clap(short, long, value_parser)]
        /// Prefix to write sorted files to
        /// if single-end [default: stdout]
        /// if paired-end [default: sorted]
        prefix: Option<String>,

        #[clap(short, long, value_parser, default_value = "true")]
        /// Whether to gzip the output files
        gzip: bool,

        #[clap(short, long, value_parser, default_value = "false")]
        /// Whether to sort by R1 or R2
        sort_by_r1: bool,
    },

    /// Extracts the transcript to gene mapping from an ensembl cdna fasta file
    T2g {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to fix
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long)]
        /// Whether to include the gene symbol in the output if available.
        /// Defaults to ensembl gene id
        symbol: bool,

        #[clap(short, long)]
        /// Whether to include the dot version of the transcript id
        /// Defaults to clipping the dot version
        dot_version: bool,
    },

    /// Trims adapter sequences that are dynamically placed within the sequence.
    Trim {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to trim sequences
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Adapater sequence to trim
        adapter: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser, default_value = "false")]
        /// Trim the adapter off the sequence
        trim_adapter: bool,
    },

    /// Filters the Fastx file for Unique Sequences
    Unique {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Filter on Unique / Duplicate Sequences
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write unique records to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write unique records to
        null: Option<String>,

        #[clap(short, long, value_parser)]
        /// Allow invalid nucleotides in output
        allow_invalid: bool,
    },

    /// Converts all lowercase nucleotides to uppercase
    /// and validates for unexpected nucleotides
    Upper {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Convert to Upper
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Allow invalid nucleotides in output
        allow_invalid: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Count { input } => {
            commands::count::run(input)?;
        }
        Commands::ExtractVariable {
            input,
            output,
            num_samples,
            zscore_threshold,
        } => {
            commands::extract::run(
                &input,
                output,
                num_samples,
                zscore_threshold,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Filter {
            input,
            output,
            pattern,
            invert,
            header,
        } => {
            commands::filter::run(
                input,
                output,
                pattern,
                invert,
                header,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Fix { input, output } => {
            commands::fix::run(
                input,
                output,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Reverse { input, output } => {
            commands::reverse::run(
                input,
                output,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Sample {
            input,
            output,
            frequency,
            seed,
            quiet,
        } => {
            commands::sample::run(
                input,
                output,
                frequency,
                seed,
                quiet,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::SgrnaTable {
            input,
            output,
            include_sequence,
            delim,
            reorder,
        } => {
            commands::sgrna_table::run(input, output, include_sequence, delim, reorder)?;
        }
        Commands::Sort {
            r1,
            r2,
            prefix,
            gzip,
            sort_by_r1,
        } => {
            commands::sort::run(
                r1,
                r2,
                prefix,
                gzip,
                sort_by_r1,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::T2g {
            input,
            output,
            symbol,
            dot_version,
        } => {
            commands::t2g::run(
                input,
                output,
                symbol,
                dot_version,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Trim {
            input,
            output,
            adapter,
            trim_adapter,
        } => {
            commands::trim::run(
                input,
                &adapter,
                output,
                trim_adapter,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Unique {
            input,
            output,
            null,
            allow_invalid,
        } => {
            commands::unique::run(
                input,
                output,
                null,
                cli.compression_threads,
                cli.compression_level,
                allow_invalid,
            )?;
        }
        Commands::Upper {
            input,
            output,
            allow_invalid,
        } => {
            commands::upper::run(
                input,
                output,
                cli.compression_threads,
                cli.compression_level,
                allow_invalid,
            )?;
        }
    };

    Ok(())
}
