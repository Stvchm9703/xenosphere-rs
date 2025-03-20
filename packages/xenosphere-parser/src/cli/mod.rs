use anyhow::Error;
// use std::ffi::OsStr;
// use std::ffi::OsString;
use clap::{Args, Parser, Subcommand, ValueEnum};
use polars::prelude::sort::arg_bottom_k;
// use fix::fix;
// use fmt::fmt;
// use lint::lint;
mod parse;
use crate::parse::parse;
mod watch;
use crate::watch::watch;
// use super::parse::parse;
use std::path::PathBuf;
/// A fictional versioning CLI
#[derive(Debug, clap::Parser)] // requires `derive` feature
#[command(
    name = "xenoshpere-parser",
    about = "A fictional versioning CLI",
    long_about = None,
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    // #[command(arg_required_else_help = true)]
    Fix(CommonArgs),

    // #[command(arg_required_else_help = true)]
    Watch(CommonArgs),

    // #[command(arg_required_else_help = true)]
    Lint(CommonArgs),

    // #[command(arg_required_else_help = true)]
    Fmt(CommonArgs),

    // #[command(arg_required_else_help = true)]
    Parse(CommonArgs),

    Check(CommonArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct CommonArgs {
    #[arg(short = 'd', long, value_name = "File or Directory")]
    files: Vec<PathBuf>,

    // #[arg(short, long, value_name = "DIRECTORY")]
    // directory: PathBuf,
    #[arg(short, long, value_name = "Config", default_value = "package.toml")]
    config: PathBuf,

    #[arg(
        short = 'k',
        long,
        value_name = "Cache Path",
        default_value = ".xs/parser"
    )]
    cache_path: PathBuf,

    #[arg(short = 'v', long, value_name = "Debug", default_value = "false")]
    is_debug: bool,

    #[arg(short, long, value_name = "Output")]
    output: Option<PathBuf>,

    #[arg(short = 'f', long, value_name = "Format", default_value = "json")]
    format: String,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        // Commands::Watch(args) => watch(&args),
        // Commands::Fix(args) => fix(&args),
        // Commands::Lint(args) => lint(&args),
        // Commands::Fmt(args) => fmt(&args),
        Commands::Parse(args) => { 
            match parse(&args) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error parsing: {:?}", err);
                    return Err(err);
                }
            };
        }
        _ => panic!("Unknown command"),
    }
    Ok(())
}
