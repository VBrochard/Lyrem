mod cli;

use crate::cli::print_error;
use crate::cli::print_metadata;
use crate::cli::print_program_headers;
use clap::Parser;
use lyrem::elf::parser::parse_elf;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Security analysis tool for Linux ELF binaries")]
struct Cli {
    path: PathBuf,
    #[arg(long)]
    json: bool,
}
fn main() {
    let cli = Cli::parse();
    let chemin = Path::new(&cli.path);
    if cli.json {
        match parse_elf(chemin) {
            Ok(metadata) => {
                if cli.json {
                    match serde_json::to_string_pretty(&metadata) {
                        Ok(json) => println!("{json}"),
                        Err(error) => eprintln!("JSON error: {error}"),
                    }
                } else {
                    print_metadata(&metadata);
                    print_program_headers(&metadata);
                }
            }
            Err(error) => print_error(&error),
        }
    } else {
        match parse_elf(chemin) {
            Ok(metadata) => {
                print_metadata(&metadata);
                print_program_headers(&metadata);
            }
            Err(error) => print_error(&error),
        }
    }
}
