use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use gb_core::Header;

#[derive(Parser)]
#[command(author, version, about = "Game Boy emulator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Info {
        rom: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Command::Info { rom, json } => info(rom, *json),
    }
}

fn info(path: &Path, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    let header = Header::read(&bytes)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&header)?);
    } else {
        println!("{header}");
    }
    Ok(())
}
