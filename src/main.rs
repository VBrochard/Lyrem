mod cli;

use lyrem::elf::parser::parse_elf;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let chemin = Path::new(&query);
    match parse_elf(chemin) {
        Ok(metadata) => cli::print_metadata(&metadata),
        Err(error) => cli::print_error(&error),
    }
}
