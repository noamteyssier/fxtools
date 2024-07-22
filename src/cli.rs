use clap::{Parser, Subcommand};

use crate::commands::csv::Delimiter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    /// Compression threads to use for output files if applicable
    #[clap(global = true, short = 'j', long)]
    pub compression_threads: Option<usize>,

    /// Compression level to use for output files if applicable
    #[clap(global = true, short = 'Z', long)]
    pub compression_level: Option<usize>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Concatenates multiple Fastx files together
    Cat {
        #[clap(short, long, value_parser, num_args=1.., required = true)]
        /// Input FASTX to concatenate
        inputs: Vec<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser, conflicts_with = "headers_only")]
        /// Selectively only write sequences of records
        sequence_only: bool,

        #[clap(short = 'H', long, value_parser, conflicts_with = "sequence_only")]
        /// Selectively only write headers of records
        headers_only: bool,

        /// Concatenate the sequences into a single line
        #[clap(
            short = 'S',
            long,
            value_parser,
            conflicts_with = "headers_only",
            requires = "sequence_only"
        )]
        single_line: bool,
    },

    /// Counts the number of records in a Fastx file
    Count {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Count
        input: Option<String>,
    },

    /// Clip nucleotide sequences between two indices
    Clip {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to clip
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Number of nucleotides from the start of the sequence to clip
        start: Option<usize>,

        #[clap(short, long, value_parser)]
        /// Number of nucleotides from the end of the sequence to clip
        end: Option<usize>,

        #[clap(short, long, value_parser, conflicts_with_all = &["start", "end"])]
        /// Range of nucleotides to accept (everything else is clipped)
        /// Format: [start]..[end]
        range: Option<String>,
    },

    /// Converts a CSV file to a FASTA file
    CsvToFasta {
        #[clap(short, long, value_parser)]
        /// Input CSV to Convert
        input: Option<String>,

        /// Filepath to write output to [default: stdout]
        /// If not provided, will write to stdout
        #[clap(short, long, value_parser)]
        output: Option<String>,

        /// Column to use as the header
        #[clap(short, long, value_parser)]
        header_col: String,

        /// Column to use as the sequence
        #[clap(short, long, value_parser)]
        sequence_col: String,

        /// Delimiter used in the CSV file
        #[clap(short, long, value_parser, default_value = "comma")]
        delim: Delimiter,
    },

    /// Create all unambiguous one-off sequences for a collection of sequences
    Disambiseq {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to disambiguate
        input: Option<String>,
        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,
        #[clap(short = 'p', long, value_parser, default_value = "false")]
        /// Include the original (parent) sequence in the output
        include_parents: bool,
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

        #[clap(short, long, value_parser, default_value = "0.5")]
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

    /// Multiplex a set of fastx files by prepending a barcode to the sequences
    Multiplex {
        #[clap(short, long, value_parser, num_args=1.., required = true)]
        /// Input FASTXs to multiplex
        input: Vec<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Optional whitelist of barcodes to prepend generated barcodes with
        whitelist: Option<String>,

        #[clap(
            short = 'O',
            long,
            value_parser,
            default_value = "multiplex_whitelist.txt"
        )]
        /// Output whitelist of barcodes to file
        output_whitelist: String,

        #[clap(short, long, value_parser, default_value = "multiplex_log.json")]
        /// Filepath to write barcode stats to
        log: String,

        #[clap(short, long, value_parser)]
        /// The size of the barcode to prepend to the sequences (will be adjusted to minimum
        /// barcode size if too small)
        barcode_size: Option<usize>,

        #[clap(short, long, value_parser)]
        /// The random seed to use for the barcode generation
        seed: Option<u64>,

        #[clap(short, long, value_parser, default_value = "100000")]
        timeout: u64,
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

        #[clap(short, long)]
        /// Ignore TSS information in the header, default is to separate by TSS
        tss_ignore: bool,

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

    /// Takes exactly a number of records from an input fastx file
    Take {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to take records from
        input: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write taken records to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Number of records to take
        num_records: usize,

        #[clap(short, long, value_parser, default_value = "0")]
        /// How many records to skip before taking the first n
        skip: usize,
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
