use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cat {
            inputs,
            output,
            sequence_only,
            headers_only,
            single_line,
        } => {
            commands::cat::run(
                inputs,
                output,
                sequence_only,
                headers_only,
                single_line,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Commands::Count { input } => {
            commands::count::run(input)?;
        }
        Commands::Clip {
            input,
            output,
            start,
            end,
            range,
        } => {
            commands::clip::run(
                input,
                output,
                start,
                end,
                range,
                cli.compression_threads,
                cli.compression_level,
            )?;
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
        Commands::Multiplex {
            input,
            output,
            whitelist,
            output_whitelist,
            log,
            barcode_size,
            seed,
            timeout,
        } => {
            commands::multiplex::run(
                input,
                output,
                whitelist,
                output_whitelist,
                log,
                barcode_size,
                seed,
                timeout,
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
        Commands::Take {
            input,
            output,
            num_records,
            skip,
        } => {
            commands::take::run(
                input,
                output,
                num_records,
                skip,
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
