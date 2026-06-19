use lyrem::elf::parser::parse_elf;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let chemin = Path::new(&query);
    let response = parse_elf(chemin);
    match response {
        Ok(metadata) => {
            println!("File Information");
            println!("----------------");
            println!("Name         : {}", metadata.file.name);
            println!("Path         : {}", metadata.file.path);
            println!("Size         : {} bytes", metadata.file.size);

            println!();
            println!("ELF Header");
            println!("----------");
            println!("Class        : {:?}", metadata.header.class);
            println!("Endianess    : {:?}", metadata.header.endianess);
            println!("ABI          : {:?}", metadata.header.abi);
            println!("Architecture : {:?}", metadata.header.architecture);
            println!("Type         : {:?}", metadata.header.binary_type);
            println!("Entry Point  : 0x{:X}", metadata.header.entry_point);
        }

        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
