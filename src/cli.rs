use lyrem::elf::metadata::ElfMetadata;
use lyrem::elf::parser::ElfError;
use lyrem::elf::security::SecurityAnalysis;

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

pub fn print_program_headers(metadata: &ElfMetadata) {
    println!();
    println!("Program Headers");
    println!("---------------");
    println!("Count        : {}", metadata.prog_header.len());

    for (index, header) in metadata.prog_header.iter().enumerate() {
        println!();
        println!(
            "Program Header {}/{}",
            index + 1,
            metadata.prog_header.len()
        );
        println!("Type         : {}", header.program_type);
        println!("Flags        : {}", header.flags);
        println!("Offset       : 0x{:X}", header.offset);
        println!("Virt. Addr   : 0x{:X}", header.virtual_address);
        println!("Phys. Addr   : 0x{:X}", header.physical_address);
        println!("File Size    : {} bytes", header.file_size);
        println!("Memory Size  : {} bytes", header.memory_size);
        println!("Align        : 0x{:X}", header.align);
    }
}

pub fn print_security_analysis(analysis: &SecurityAnalysis) {
    println!();
    println!("Security Analysis");
    println!("-----------------");
    println!("NX               : {}", analysis.nx);
    println!("PIE              : {}", analysis.pie);
    println!(
        "Dynamic Segment  : {}",
        if analysis.has_dynamic_segment {
            "Yes"
        } else {
            "No"
        }
    );
    println!("RELRO            : {}", analysis.relro);
    println!(
        "Interpreter      : {}",
        if analysis.has_interpreter {
            "Yes"
        } else {
            "No"
        }
    );

    if analysis.rwx_segment.is_empty() {
        println!("RWX Segments     : None");
    } else {
        println!("RWX Segments     : {}", analysis.rwx_segment.len());

        for (index, segment) in analysis.rwx_segment.iter().enumerate() {
            println!();
            println!("RWX Segment {}/{}", index + 1, analysis.rwx_segment.len());
            println!("Type             : {}", segment.program_type);
            println!("Flags            : {}", segment.flags);
            println!("Offset           : 0x{:X}", segment.offset);
            println!("Virt. Addr       : 0x{:X}", segment.virtual_address);
            println!("File Size        : {} bytes", segment.file_size);
            println!("Memory Size      : {} bytes", segment.memory_size);
        }
    }
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

        ElfError::BadDynamicEntry => {
            eprintln!("Error: Much Dynamic Entry");
        }
    }
}
