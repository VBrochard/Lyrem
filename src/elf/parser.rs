use std::fs::File;
use std::fs::*;
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::Path;

use crate::elf::dynamic::DynamicEntry;
use crate::elf::dynamic::DynamicTag;
use crate::elf::dynamic::DynamicTag::BindNow;
use crate::elf::dynamic::DynamicTag::Flags;
use crate::elf::dynamic::DynamicTag::Flags1;
use crate::elf::dynamic::DynamicTag::Needed;
use crate::elf::dynamic::DynamicTag::Null;
use crate::elf::dynamic::DynamicTag::RPath;
use crate::elf::dynamic::DynamicTag::RunPath;
use crate::elf::dynamic::DynamicTag::Soname;
use crate::elf::dynamic::DynamicTag::StrSz;
use crate::elf::dynamic::DynamicTag::StrTab;
use crate::elf::dynamic::DynamicTag::Unknown;
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
use crate::elf::metadata::Endianness::BigEndian;
use crate::elf::metadata::Endianness::LittleEndian;
use crate::elf::metadata::Endianness::{self};
use crate::elf::metadata::FileMetadata;
use crate::elf::parser::ElfError::BadHeader;
use crate::elf::parser::ElfError::NotAnElfFile;

use crate::elf::program::ProgramFlags;
use crate::elf::program::ProgramHeader;
use crate::elf::program::ProgramType;
use crate::elf::program::ProgramType::Dynamic;
use crate::elf::program::ProgramType::GnuEhFrame;
use crate::elf::program::ProgramType::GnuProperty;
use crate::elf::program::ProgramType::GnuRelro;
use crate::elf::program::ProgramType::GnuStack;
use crate::elf::program::ProgramType::Interp;
use crate::elf::program::ProgramType::Load;
use crate::elf::program::ProgramType::Note;
use crate::elf::program::ProgramType::Phdr;
use crate::elf::program::ProgramType::Shlib;
use crate::elf::program::ProgramType::Tls;

/// Errors that can occur while parsing an ELF file.
#[derive(Debug)]
pub enum ElfError {
    /// Underlying I/O error while reading the file.
    Io(io::Error),
    /// The file does not start with the ELF magic number.
    NotAnElfFile,
    /// The ELF header or one of its mandatory fields is malformed.
    BadHeader,
    /// A Dynamic Section entry could not be parsed.
    BadDynamicEntry,
}

impl From<io::Error> for ElfError {
    fn from(error: io::Error) -> ElfError {
        ElfError::Io(error)
    }
}

/// Parses a Linux ELF file and returns Lyrem's structured metadata.
///
/// This is the main parsing entry point. It reads the ELF header, Program
/// Headers, and Dynamic Section entries, then returns a structured
/// representation used by the CLI and security analysis layers.
pub fn parse_elf(path: &Path) -> Result<ElfMetadata, ElfError> {
    let mut f = File::open(path)?;
    check_magic(&mut f)?;
    let class_byte = read_u8(&mut f)?;
    let class = parse_class(class_byte);
    let endian_byte = read_u8(&mut f)?;
    let endianness = parse_endian(endian_byte)?;
    read_u8(&mut f)?;
    let abi_byte = read_u8(&mut f)?;
    let abi = parse_abi(abi_byte);
    for _ in 0..8 {
        read_u8(&mut f)?;
    }
    let binary_byte = read_u16(&mut f, &endianness)?;
    let binary_type = parse_binary_type(binary_byte);
    let archi_byte = read_u16(&mut f, &endianness)?;
    let architecture = parse_architecture(archi_byte);
    for _ in 0..4 {
        read_u8(&mut f)?;
    }
    let entry_point = match &class {
        ElfClass::Elf32 => read_u32(&mut f, &endianness)? as u64,
        ElfClass::Elf64 => read_u64(&mut f, &endianness)?,
        ElfClass::Unknown(_) => return Err(BadHeader),
    };
    let e_phoff = match &class {
        ElfClass::Elf32 => read_u32(&mut f, &endianness)? as u64,
        ElfClass::Elf64 => read_u64(&mut f, &endianness)?,
        ElfClass::Unknown(_) => return Err(BadHeader),
    };
    let _e_shoff = match &class {
        ElfClass::Elf32 => read_u32(&mut f, &endianness)? as u64,
        ElfClass::Elf64 => read_u64(&mut f, &endianness)?,
        ElfClass::Unknown(_) => return Err(BadHeader),
    };
    let _e_flags = read_u32(&mut f, &endianness)?;
    let _e_ehsize = read_u16(&mut f, &endianness)?;
    let _e_phentsize = read_u16(&mut f, &endianness)?;
    let e_phnum = read_u16(&mut f, &endianness)?;
    f.seek(SeekFrom::Start(e_phoff))?;
    let mut prog_header: Vec<ProgramHeader> = Vec::new();
    for _ in 0..e_phnum {
        prog_header.push(parse_program_header(&mut f, &endianness, &class)?);
    }
    let dyn_entry = parse_dynamic(&prog_header, &mut f, &endianness, &class)?;
    let header = ElfHeaderMetadata {
        class,
        endianness,
        abi,
        architecture,
        binary_type,
        entry_point,
    };
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let clean_path = path.to_string_lossy().to_string();
    let size = metadata(path)?.len();
    let file = FileMetadata {
        name,
        clean_path,
        size,
    };
    let info_header = ElfMetadata {
        file,
        header,
        prog_header,
        dyn_entry,
    };
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

fn parse_endian(endian: u8) -> Result<Endianness, ElfError> {
    match endian {
        1 => Ok(Endianness::LittleEndian),
        2 => Ok(Endianness::BigEndian),
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

fn read_u16(desc: &mut File, endia: &Endianness) -> Result<u16, ElfError> {
    let mut buffer = [0; 2];
    desc.read_exact(&mut buffer)?;
    let response = match endia {
        LittleEndian => u16::from_le_bytes(buffer),
        BigEndian => u16::from_be_bytes(buffer),
    };
    Ok(response)
}

fn read_u32(desc: &mut File, endia: &Endianness) -> Result<u32, ElfError> {
    let mut buffer = [0; 4];
    desc.read_exact(&mut buffer)?;
    let response = match endia {
        LittleEndian => u32::from_le_bytes(buffer),
        BigEndian => u32::from_be_bytes(buffer),
    };
    Ok(response)
}

fn read_u64(desc: &mut File, endia: &Endianness) -> Result<u64, ElfError> {
    let mut buffer = [0; 8];
    desc.read_exact(&mut buffer)?;
    let response = match endia {
        LittleEndian => u64::from_le_bytes(buffer),
        BigEndian => u64::from_be_bytes(buffer),
    };
    Ok(response)
}

fn parse_program_type(byte: u32) -> ProgramType {
    match byte {
        0 => ProgramType::Null,
        1 => Load,
        2 => Dynamic,
        3 => Interp,
        4 => Note,
        5 => Shlib,
        6 => Phdr,
        7 => Tls,
        1685382480 => GnuEhFrame,
        1685382481 => GnuStack,
        1685382482 => GnuRelro,
        1685382483 => GnuProperty,
        x => ProgramType::Unknown(x),
    }
}

fn parse_flags(byte: u32) -> ProgramFlags {
    ProgramFlags {
        readable: (byte & 4) != 0,
        writable: (byte & 2) != 0,
        executable: (byte & 1) != 0,
    }
}

fn parse_program_header(
    desc: &mut File,
    endia: &Endianness,
    class: &ElfClass,
) -> Result<ProgramHeader, ElfError> {
    // ELF32 and ELF64 Program Headers store `p_flags` at different offsets.
    match class {
        ElfClass::Elf32 => {
            let ptype = parse_program_type(read_u32(desc, endia)?);
            let poffset = read_u32(desc, endia)? as u64;
            let pvaddr = read_u32(desc, endia)? as u64;
            let ppaddr = read_u32(desc, endia)? as u64;
            let pfilesz = read_u32(desc, endia)? as u64;
            let pmemsz = read_u32(desc, endia)? as u64;
            let pflags = parse_flags(read_u32(desc, endia)?);
            let palign = read_u32(desc, endia)? as u64;

            let ph = ProgramHeader {
                program_type: ptype,
                flags: pflags,
                offset: poffset,
                virtual_address: pvaddr,
                physical_address: ppaddr,
                file_size: pfilesz,
                memory_size: pmemsz,
                align: palign,
            };
            Ok(ph)
        }
        ElfClass::Elf64 => {
            let ptype = parse_program_type(read_u32(desc, endia)?);
            let pflags = parse_flags(read_u32(desc, endia)?);
            let poffset = read_u64(desc, endia)?;
            let pvaddr = read_u64(desc, endia)?;
            let ppaddr = read_u64(desc, endia)?;
            let pfilesz = read_u64(desc, endia)?;
            let pmemsz = read_u64(desc, endia)?;
            let palign = read_u64(desc, endia)?;
            let ph = ProgramHeader {
                program_type: ptype,
                flags: pflags,
                offset: poffset,
                virtual_address: pvaddr,
                physical_address: ppaddr,
                file_size: pfilesz,
                memory_size: pmemsz,
                align: palign,
            };
            Ok(ph)
        }
        ElfClass::Unknown(_) => Err(ElfError::BadHeader),
    }
}

fn parse_tag(byte: i64) -> DynamicTag {
    match byte {
        0 => Null,
        1 => Needed,
        5 => StrTab,
        10 => StrSz,
        14 => Soname,
        15 => RPath,
        24 => BindNow,
        29 => RunPath,
        30 => Flags,
        1879048187 => Flags1,
        x => Unknown(x),
    }
}

fn parse_dynamic(
    prog: &[ProgramHeader],
    desc: &mut File,
    endia: &Endianness,
    class: &ElfClass,
) -> Result<Vec<DynamicEntry>, ElfError> {
    let mut dynamic_header = None;
    let mut dyn_entry: Vec<DynamicEntry> = Vec::new();
    for header in prog {
        if header.program_type == ProgramType::Dynamic {
            dynamic_header = Some(header);
            break;
        }
    }
    let dynamic_header = match dynamic_header {
        Some(header) => header,
        None => return Ok(Vec::new()),
    };
    desc.seek(SeekFrom::Start(dynamic_header.offset))?;
    match class {
        ElfClass::Elf32 => {
            let mut byte_read = 0;
            while byte_read < dynamic_header.file_size {
                let tag = parse_tag(read_u32(desc, endia)? as i64);
                if tag == DynamicTag::Null {
                    break;
                }
                let entry = DynamicEntry {
                    tag,
                    value: read_u32(desc, endia)? as u64,
                };
                byte_read += 8;
                dyn_entry.push(entry);
            }
            Ok(dyn_entry)
        }
        ElfClass::Elf64 => {
            let mut byte_read = 0;
            while byte_read < dynamic_header.file_size {
                let tag = parse_tag(read_u64(desc, endia)? as i64);
                if tag == DynamicTag::Null {
                    break;
                }
                let entry = DynamicEntry {
                    tag,
                    value: read_u64(desc, endia)?,
                };
                byte_read += 16;
                dyn_entry.push(entry);
            }
            Ok(dyn_entry)
        }
        ElfClass::Unknown(_) => Err(ElfError::BadDynamicEntry),
    }
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
        assert!(matches!(parse_endian(1), Ok(Endianness::LittleEndian)));
        assert!(matches!(parse_endian(2), Ok(Endianness::BigEndian)));
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

    #[test]
    fn test_parse_tag() {
        assert_eq!(parse_tag(0), DynamicTag::Null);
        assert_eq!(parse_tag(1), DynamicTag::Needed);
        assert_eq!(parse_tag(5), DynamicTag::StrTab);
        assert_eq!(parse_tag(10), DynamicTag::StrSz);
        assert_eq!(parse_tag(14), DynamicTag::Soname);
        assert_eq!(parse_tag(15), DynamicTag::RPath);
        assert_eq!(parse_tag(24), DynamicTag::BindNow);
        assert_eq!(parse_tag(29), DynamicTag::RunPath);
        assert_eq!(parse_tag(30), DynamicTag::Flags);
        assert_eq!(parse_tag(1879048187), DynamicTag::Flags1);
        assert_eq!(parse_tag(458), DynamicTag::Unknown(458));
    }
}
