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

        #[clap(short, long, value_parser)]
        delim: Option<char>,

    }
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Unique { input, output, null } => { 
            commands::unique::run(input, output, null)?; 
        },
        Commands::SgrnaTable { input, output, delim } => {
            commands::sgrna_table::run(input, output, delim)?;
        }
    };

    Ok(())
}
