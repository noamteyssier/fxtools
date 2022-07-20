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
        /// Input FAST[AQ] to Filter on Unique / Duplicate Sequences
        input: String,

        #[clap(short, long, value_parser)]
        /// Filepath to write unique records to [default: stdout]
        output: Option<String>,

        #[clap(short, long, value_parser)]
        /// Filepath to write unique records to
        null: Option<String>

    }
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Unique { input, output, null } => { 
            commands::unique::run(input, output, null)?; 
        }
    };

    Ok(())
}
