mod cli;

use crate::cli::print_error;
use crate::cli::print_metadata;
use crate::cli::print_program_headers;
use crate::cli::print_security_analysis;
use clap::Parser;
use lyrem::elf::parser::parse_elf;
use lyrem::elf::security::analyze;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Security analysis tool for Linux ELF binaries")]
struct Cli {
    /// ELF binary to analyze
    path: PathBuf,
    /// Output analysis as JSON
    #[arg(long)]
    json: bool,
    /// Display the full ELF report
    #[arg(long)]
    full: bool,
}
fn main() {
    let cli = Cli::parse();
    let chemin = Path::new(&cli.path);
    match parse_elf(chemin) {
        Ok(metadata) => {
            let analyse = analyze(&metadata);
            if cli.json {
                match serde_json::to_string_pretty(&metadata) {
                    Ok(json) => println!("{json}"),
                    Err(error) => eprintln!("JSON error: {error}"),
                }
            } else if cli.full {
                print_metadata(&metadata);
                print_security_analysis(&analyse);
                print_program_headers(&metadata);
            } else {
                print_security_analysis(&analyse);
            }
        }
        Err(error) => print_error(&error),
    }
}
