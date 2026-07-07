mod cli;

use crate::cli::print_error;
use crate::cli::print_metadata;
use lyrem::elf::parser::parse_elf;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let chemin = Path::new(&query);
    match parse_elf(chemin) {
        Ok(metadata) => {
            print_metadata(&metadata);
            println!("\nProgram Headers:");
            println!("{:#?}", metadata.prog_header);
        }
        Err(error) => print_error(&error),
    }
}
