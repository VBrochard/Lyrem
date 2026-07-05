use std::fs::File;
use std::fs::*;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::elf::metadata::Abi;
use crate::elf::metadata::Architecture;
use crate::elf::metadata::Architecture::AArch64;
use crate::elf::metadata::Architecture::Arm;

use crate::elf::metadata::Architecture::RiscV;
use crate::elf::metadata::Architecture::Sparc;

use crate::elf::metadata::Architecture::X86;
use crate::elf::metadata::Architecture::X86_64;
use crate::elf::metadata::BinaryType;

use crate::elf::metadata::BinaryType::Core;
use crate::elf::metadata::BinaryType::Executable;

use crate::elf::metadata::BinaryType::Relocatable;
use crate::elf::metadata::BinaryType::SharedObject;

use crate::elf::metadata::ElfClass::{self};
use crate::elf::metadata::ElfHeaderMetadata;
use crate::elf::metadata::ElfMetadata;
use crate::elf::metadata::Endianess::BigEndian;
use crate::elf::metadata::Endianess::LittleEndian;
use crate::elf::metadata::Endianess::{self};
use crate::elf::metadata::FileMetadata;
use crate::elf::parser::ElfError::BadHeader;
use crate::elf::parser::ElfError::NotAnElfFile;

#[derive(Debug)]
pub enum ElfError {
    Io(io::Error),
    NotAnElfFile,
    BadHeader,
}

impl From<io::Error> for ElfError {
    fn from(error: io::Error) -> ElfError {
        ElfError::Io(error)
    }
}

pub fn parse_elf(chemin: &Path) -> Result<ElfMetadata, ElfError> {
    let mut f = File::open(chemin)?;
    check_magic(&mut f)?;
    let class_byte = read_u8(&mut f)?;
    let class = parse_class(class_byte);
    let endian_byte = read_u8(&mut f)?;
    let endianess = parse_endian(endian_byte)?;
    read_u8(&mut f)?;
    let abi_byte = read_u8(&mut f)?;
    let abi = parse_abi(abi_byte);
    for _ in 0..8 {
        read_u8(&mut f)?;
    }
    let binary_byte = read_u16(&mut f, &endianess)?;
    let binary_type = parse_binary_type(binary_byte);
    let archi_byte = read_u16(&mut f, &endianess)?;
    let architecture = parse_architecture(archi_byte);
    for _ in 0..4 {
        read_u8(&mut f)?;
    }
    let entry_point = match &class {
        ElfClass::Elf32 => read_u32(&mut f, &endianess)? as u64,
        ElfClass::Elf64 => read_u64(&mut f, &endianess)?,
        ElfClass::Unknown(_) => return Err(BadHeader),
    };

    let header = ElfHeaderMetadata {
        class,
        endianess,
        abi,
        architecture,
        binary_type,
        entry_point,
    };
    let name = chemin.file_name().unwrap().to_string_lossy().to_string();
    let path = chemin.to_string_lossy().to_string();
    let size = metadata(chemin)?.len();

    let file = FileMetadata { name, path, size };
    let info_header = ElfMetadata { file, header };
    Ok(info_header)
}

fn check_magic(desc: &mut File) -> Result<(), ElfError> {
    let magic_tab = [127, 69, 76, 70];
    let mut buffer = [0; 4];
    desc.read_exact(&mut buffer)?;
    if magic_tab == buffer {
        Ok(())
    } else {
        Err(NotAnElfFile)
    }
}

fn parse_class(class: u8) -> ElfClass {
    match class {
        1 => ElfClass::Elf32,
        2 => ElfClass::Elf64,
        x => ElfClass::Unknown(x),
    }
}

fn parse_endian(endian: u8) -> Result<Endianess, ElfError> {
    match endian {
        1 => Ok(Endianess::LittleEndian),
        2 => Ok(Endianess::BigEndian),
        _ => Err(BadHeader),
    }
}

fn parse_abi(abi: u8) -> Abi {
    match abi {
        0 => Abi::SystemV,
        2 => Abi::NetBSD,
        3 => Abi::Linux,
        9 => Abi::FreeBSD,
        12 => Abi::OpenBSD,
        97 => Abi::Arm,
        255 => Abi::Standalone,
        x => Abi::Unknown(x),
    }
}

fn parse_binary_type(bin: u16) -> BinaryType {
    match bin {
        0 => BinaryType::None,
        1 => Relocatable,
        2 => Executable,
        3 => SharedObject,
        4 => Core,
        x => BinaryType::Unknown(x),
    }
}

fn parse_architecture(archi: u16) -> Architecture {
    match archi {
        0 => Architecture::None,
        2 => Sparc,
        3 => X86,
        40 => Arm,
        62 => X86_64,
        183 => AArch64,
        243 => RiscV,
        x => Architecture::Unknown(x),
    }
}

fn read_u8(desc: &mut File) -> Result<u8, ElfError> {
    let mut buffer = [0];
    desc.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_u16(desc: &mut File, endia: &Endianess) -> Result<u16, ElfError> {
    let mut buffer = [0; 2];
    desc.read_exact(&mut buffer)?;
    let response = match endia {
        LittleEndian => u16::from_le_bytes(buffer),
        BigEndian => u16::from_be_bytes(buffer),
    };
    Ok(response)
}

fn read_u32(desc: &mut File, endia: &Endianess) -> Result<u32, ElfError> {
    let mut buffer = [0; 4];
    desc.read_exact(&mut buffer)?;
    let response = match endia {
        LittleEndian => u32::from_le_bytes(buffer),
        BigEndian => u32::from_be_bytes(buffer),
    };
    Ok(response)
}

fn read_u64(desc: &mut File, endia: &Endianess) -> Result<u64, ElfError> {
    let mut buffer = [0; 8];
    desc.read_exact(&mut buffer)?;
    let response = match endia {
        LittleEndian => u64::from_le_bytes(buffer),
        BigEndian => u64::from_be_bytes(buffer),
    };
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_class() {
        assert_eq!(parse_class(2), ElfClass::Elf64);
        assert_eq!(parse_class(1), ElfClass::Elf32);
        assert_eq!(parse_class(29), ElfClass::Unknown(29));
    }

    #[test]
    fn test_parse_endian() {
        assert!(matches!(parse_endian(1), Ok(Endianess::LittleEndian)));
        assert!(matches!(parse_endian(2), Ok(Endianess::BigEndian)));
        assert!(matches!(parse_endian(42), Err(ElfError::BadHeader)));
    }

    #[test]
    fn test_parse_abi() {
        assert_eq!(parse_abi(0), Abi::SystemV);
        assert_eq!(parse_abi(2), Abi::NetBSD);
        assert_eq!(parse_abi(3), Abi::Linux);
        assert_eq!(parse_abi(9), Abi::FreeBSD);
        assert_eq!(parse_abi(12), Abi::OpenBSD);
        assert_eq!(parse_abi(97), Abi::Arm);
        assert_eq!(parse_abi(255), Abi::Standalone);
        assert_eq!(parse_abi(185), Abi::Unknown(185));
    }

    #[test]
    fn test_parse_binary_type() {
        assert_eq!(parse_binary_type(0), BinaryType::None);
        assert_eq!(parse_binary_type(1), BinaryType::Relocatable);
        assert_eq!(parse_binary_type(2), BinaryType::Executable);
        assert_eq!(parse_binary_type(3), BinaryType::SharedObject);
        assert_eq!(parse_binary_type(4), BinaryType::Core);
        assert_eq!(parse_binary_type(359), BinaryType::Unknown(359));
    }

    #[test]
    fn test_parse_architecture() {
        assert_eq!(parse_architecture(0), Architecture::None);
        assert_eq!(parse_architecture(2), Architecture::Sparc);
        assert_eq!(parse_architecture(3), Architecture::X86);
        assert_eq!(parse_architecture(40), Architecture::Arm);
        assert_eq!(parse_architecture(62), Architecture::X86_64);
        assert_eq!(parse_architecture(183), Architecture::AArch64);
        assert_eq!(parse_architecture(243), Architecture::RiscV);
        assert_eq!(parse_architecture(789), Architecture::Unknown(789));
    }
}
