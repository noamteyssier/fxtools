use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    #[clap(subcommand)]
    command: Commands

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
        null: Option<String>

    },

    /// Creates a mapping of sgRNAs to their parent gene
    SgrnaTable{

        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Generate table
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write table to [default: stdout]
        output: Option<String>,

        #[clap(short='s', long, action)]
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
        output: Option<String>
    },

    /// Creates the Reverse complement for a provided fastx
    Reverse {
        #[clap(short, long, value_parser)]
        /// Input FASTA/Q to Convert to Upper
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write output to [default: stdout]
        output: Option<String>
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

        #[clap(short, long, value_parser, default_value="5000")]
        /// Number of samples to calculate positional entropy on
        num_samples: usize,

        #[clap(short, long, value_parser, default_value="1.0")]
        /// Number of samples to calculate positional entropy on
        zscore_threshold: f64
    }
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Unique { input, output, null } => { 
            commands::unique::run(&input, output, null)?; 
        },
        Commands::SgrnaTable { input, output, include_sequence, delim, reorder } => {
            commands::sgrna_table::run(&input, output, include_sequence, delim, reorder)?;
        },
        Commands::Upper { input, output } => {
            commands::upper::run(&input, output)?;
        },
        Commands::Reverse { input, output } => {
            commands::reverse::run(&input, output)?;
        },
        Commands::ExtractVariable { input, output, num_samples, zscore_threshold } => {
            commands::extract::run(&input, output, num_samples, zscore_threshold)?;
        }
    };

    Ok(())
}
