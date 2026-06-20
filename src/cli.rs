use lyrem::elf::metadata::ElfMetadata;
use lyrem::elf::parser::ElfError;

pub fn print_metadata(metadata: &ElfMetadata) {
    println!("File Information");
    println!("----------------");
    println!("Name         : {}", metadata.file.name);
    println!("Path         : {}", metadata.file.path);
    println!("Size         : {} bytes", metadata.file.size);

    println!();
    println!("ELF Header");
    println!("----------");
    println!("Class        : {}", metadata.header.class);
    println!("Endianess    : {}", metadata.header.endianess);
    println!("ABI          : {}", metadata.header.abi);
    println!("Architecture : {}", metadata.header.architecture);
    println!("Type         : {}", metadata.header.binary_type);
    println!("Entry Point  : 0x{:X}", metadata.header.entry_point);
}

pub fn print_error(error: &ElfError) {
    match error {
        ElfError::NotAnElfFile => {
            eprintln!("Error: file is not a valid ELF binary");
        }

        ElfError::BadHeader => {
            eprintln!("Error: ELF header is corrupted");
        }

        ElfError::Io(err) => {
            eprintln!("I/O Error: {}", err);
        }
    }
}
