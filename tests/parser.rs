/*
Build the generated test assets before running the test suite.
*/

#[cfg(test)]
mod tests {
    use std::path::Path;

    use lyrem::elf::metadata::*;
    use lyrem::elf::parser::*;
    use lyrem::elf::program::ProgramType;
    use lyrem::elf::security::RelroStatus;
    use lyrem::elf::security::Status;
    use lyrem::elf::security::analyze;
    #[test]
    fn test_parse_elf_hello_exe() {
        let path = Path::new("tests/assets/generated/hello");
        let result = parse_elf(path);
        assert!(result.is_ok());
        let elf = result.unwrap();
        let analyze = analyze(&elf);

        assert_eq!(elf.header.class, ElfClass::Elf64);
        assert_eq!(elf.header.architecture, Architecture::X86_64);
        assert_eq!(elf.header.binary_type, BinaryType::Executable);
        assert_eq!(analyze.nx, Status::Enabled);
        assert_eq!(analyze.pie, Status::Disabled);
        assert_eq!(analyze.relro, RelroStatus::Partial);
        assert_eq!(analyze.has_interpreter, true);
        assert_eq!(analyze.rwx_segment.len(), 0);
        assert!(
            elf.prog_header
                .iter()
                .any(|header| { header.program_type == ProgramType::Load })
        );
        assert!(
            elf.prog_header
                .iter()
                .any(|header| { header.program_type == ProgramType::Interp })
        );
        assert!(
            elf.prog_header
                .iter()
                .any(|header| { header.program_type == ProgramType::Dynamic })
        );
    }

    #[test]
    fn test_parse_elf_so() {
        let path = Path::new("tests/assets/generated/libhello.so");
        let result = parse_elf(path);
        assert!(result.is_ok());
        let elf = result.unwrap();

        assert_eq!(elf.header.class, ElfClass::Elf64);
        assert_eq!(elf.header.architecture, Architecture::X86_64);
        assert_eq!(elf.header.binary_type, BinaryType::SharedObject);
    }

    #[test]
    fn test_parse_not_elf() {
        let path = Path::new("tests/assets/not_elf.txt");
        let result = parse_elf(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_assembly() {
        let path = Path::new("tests/assets/generated/rwx");
        let result = parse_elf(path);
        assert!(result.is_ok());
        let elf = result.unwrap();
        let analyze = analyze(&elf);

        assert_eq!(elf.header.class, ElfClass::Elf64);
        assert_eq!(elf.header.architecture, Architecture::X86_64);
        assert_eq!(elf.header.binary_type, BinaryType::Executable);
        assert_eq!(elf.header.endianess, Endianess::LittleEndian);
        assert_eq!(analyze.rwx_segment.len(), 1);
        assert_eq!(analyze.has_dynamic_segment, false);
        assert_eq!(analyze.has_interpreter, false);
        assert_eq!(analyze.relro, RelroStatus::None);
    }

    #[test]
    fn test_parse_execstack() {
        let path = Path::new("tests/assets/generated/hello_execstack");
        let result = parse_elf(path);
        assert!(result.is_ok());
        let elf = result.unwrap();
        let analyze = analyze(&elf);

        assert_eq!(elf.header.class, ElfClass::Elf64);
        assert_eq!(elf.header.architecture, Architecture::X86_64);
        assert_eq!(elf.header.binary_type, BinaryType::Executable);
        assert_eq!(elf.header.endianess, Endianess::LittleEndian);
        assert_eq!(analyze.nx, Status::Disabled);
        assert_eq!(analyze.pie, Status::Disabled);
        assert_eq!(analyze.relro, RelroStatus::Partial);
        assert_eq!(analyze.has_interpreter, true);
        assert_eq!(analyze.rwx_segment.len(), 1);
    }

    #[test]
    fn test_parse_pie_partial_relro() {
        let path = Path::new("tests/assets/generated/hello_pie_partial_relro");
        let result = parse_elf(path);
        assert!(result.is_ok());
        let elf = result.unwrap();
        let analyze = analyze(&elf);

        assert_eq!(elf.header.class, ElfClass::Elf64);
        assert_eq!(elf.header.architecture, Architecture::X86_64);
        assert_eq!(elf.header.binary_type, BinaryType::SharedObject);
        assert_eq!(elf.header.endianess, Endianess::LittleEndian);
        assert_eq!(analyze.nx, Status::Enabled);
        assert_eq!(analyze.pie, Status::Enabled);
        assert_eq!(analyze.relro, RelroStatus::Partial);
        assert_eq!(analyze.has_interpreter, true);
        assert_eq!(analyze.rwx_segment.len(), 0);
    }
}
