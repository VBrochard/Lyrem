use clap::Parser;
use lyrem::elf::metadata::ElfMetadata;
use lyrem::elf::parser::parse_elf;
use lyrem::elf::security::SecurityAnalysis;
use lyrem::elf::security::analyze;
use lyrem::output::cli::print_error;
use lyrem::output::cli::print_metadata;
use lyrem::output::cli::print_program_headers;
use lyrem::output::cli::print_security_analysis;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Security analysis tool for Linux ELF binaries")]
struct Cli {
    /// ELF binary to analyze.
    path: PathBuf,
    /// Output analysis as JSON.
    #[arg(long)]
    json: bool,
    /// Display the full ELF report.
    #[arg(long)]
    full: bool,
}

#[derive(Serialize)]
struct JsonOutput<'a> {
    metadata: &'a ElfMetadata,
    security_analysis: &'a SecurityAnalysis,
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    match parse_elf(path) {
        Ok(metadata) => {
            let json = JsonOutput {
                metadata: &metadata,
                security_analysis: &analyze(&metadata),
            };
            if cli.json {
                match serde_json::to_string_pretty(&json) {
                    Ok(json) => println!("{json}"),
                    Err(error) => eprintln!("JSON error: {error}"),
                }
            } else if cli.full {
                print_metadata(&metadata);
                print_security_analysis(json.security_analysis);
                print_program_headers(&metadata);
            } else {
                print_security_analysis(json.security_analysis);
            }
        }
        Err(error) => print_error(&error),
    }
}
