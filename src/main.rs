use lyrem::elf::parser::parse_elf;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let chemin = Path::new(&query);
    let response = parse_elf(&chemin);
    match response {
        Ok(header) => println!(
            "Class        : {:?}
Endianess    : {:?}
ABI          : {:?}
Architecture : {:?}
Type         : {:?}
Entry Point  : 0x{:X}",
            header.class,
            header.endianess,
            header.abi,
            header.architecture,
            header.binary_type,
            header.entry_point
        ),
        Err(x) => println!("Error : {:?}", x),
    }
}
