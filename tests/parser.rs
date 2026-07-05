#[cfg(test)]
mod tests {
    use std::path::Path;

    use lyrem::elf::metadata::*;
    use lyrem::elf::parser::*;
    #[test]
    fn test_parse_elf_exe() {
        let path = Path::new("tests/assets/hello");
        let result = parse_elf(path);
        assert!(result.is_ok());
        let elf = result.unwrap();

        assert_eq!(elf.header.class, ElfClass::Elf64);
        assert_eq!(elf.header.architecture, Architecture::X86_64);
        assert_eq!(elf.header.binary_type, BinaryType::Executable);
    }

    #[test]
    fn test_parse_elf_so() {
        let path = Path::new("tests/assets/libhello.so");
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
}
