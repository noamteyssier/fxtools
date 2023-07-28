use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Filters the Fastx file for Unique Sequences
    Unique {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Filter on Unique / Duplicate Sequences
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write unique records to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write unique records to
        null: Option<String>,

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,

        #[clap(short, long, value_parser)]
        /// Allow invalid nucleotides in output
        allow_invalid: bool,
    },

    /// Creates a mapping of sgRNAs to their parent gene
    SgrnaTable {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Generate table
        input: String,

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

    /// Converts all lowercase nucleotides to uppercase
    /// and validates for unexpected nucleotides
    Upper {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Convert to Upper
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,

        #[clap(short, long, value_parser)]
        /// Allow invalid nucleotides in output
        allow_invalid: bool,
    },

    /// Creates the Reverse complement for a provided fastx
    Reverse {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Convert to Upper
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,
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

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,
    },

    /// Trims adapter sequences that are dynamically placed within the sequence.
    Trim {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to trim sequences
        input: String,

        #[clap(short, long, value_parser)]
        /// Adapater sequence to trim
        adapter: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser, default_value = "false")]
        /// Trim the adapter off the sequence
        trim_adapter: bool,

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,
    },

    /// Sorts a fastx file by sequence
    Sort {
        #[clap(short = 'i', long, value_parser)]
        /// Input FASTA/Q to sort
        r1: String,

        #[clap(short = 'I', long, value_parser)]
        /// Optional choice of R2 to sort by
        r2: Option<String>,

        #[clap(short, long, value_parser, default_value = "sorted")]
        /// Prefix to write sorted files to
        prefix: String,

        #[clap(short, long, value_parser, default_value = "true")]
        /// Whether to gzip the output files
        gzip: bool,

        #[clap(short, long, value_parser, default_value = "false")]
        /// Whether to sort by R1 or R2
        sort_by_r1: bool,

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,
    },

    /// Fix a fastx file by replacing invalid characters with N
    Fix {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to fix
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short = 'j', long, value_parser)]
        /// Number of threads to use in gzip compression
        num_threads: Option<usize>,

        #[clap(short = 'Z', long, value_parser)]
        /// gzip compression level
        compression_level: Option<usize>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Unique {
            input,
            output,
            null,
            num_threads,
            compression_level,
            allow_invalid,
        } => {
            commands::unique::run(
                &input,
                output,
                null,
                num_threads,
                compression_level,
                allow_invalid,
            )?;
        }
        Commands::SgrnaTable {
            input,
            output,
            include_sequence,
            delim,
            reorder,
        } => {
            commands::sgrna_table::run(&input, output, include_sequence, delim, reorder)?;
        }
        Commands::Upper {
            input,
            output,
            num_threads,
            compression_level,
            allow_invalid,
        } => {
            commands::upper::run(
                &input,
                output,
                num_threads,
                compression_level,
                allow_invalid,
            )?;
        }
        Commands::Reverse {
            input,
            output,
            num_threads,
            compression_level,
        } => {
            commands::reverse::run(&input, output, num_threads, compression_level)?;
        }
        Commands::ExtractVariable {
            input,
            output,
            num_samples,
            zscore_threshold,
            num_threads,
            compression_level,
        } => {
            commands::extract::run(
                &input,
                output,
                num_samples,
                zscore_threshold,
                num_threads,
                compression_level,
            )?;
        }
        Commands::Trim {
            input,
            output,
            adapter,
            trim_adapter,
            num_threads,
            compression_level,
        } => {
            commands::trim::run(
                &input,
                &adapter,
                output,
                trim_adapter,
                num_threads,
                compression_level,
            )?;
        }
        Commands::Sort {
            r1,
            r2,
            prefix,
            gzip,
            sort_by_r1,
            num_threads,
            compression_level,
        } => {
            commands::sort::run(
                &r1,
                r2,
                &prefix,
                gzip,
                sort_by_r1,
                num_threads,
                compression_level,
            )?;
        }
        Commands::Fix {
            input,
            output,
            num_threads,
            compression_level,
        } => {
            commands::fix::run(&input, output, num_threads, compression_level)?;
        }
    };

    Ok(())
}
