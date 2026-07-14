use std::fmt;

use serde::Serialize;

use crate::elf::{dynamic::DynamicEntry, program::ProgramHeader};

#[derive(Debug, PartialEq, Serialize)]
pub enum ElfClass {
    Elf32,
    Elf64,
    Unknown(u8),
}

impl fmt::Display for ElfClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfClass::Elf32 => write!(f, "ELF32"),
            ElfClass::Elf64 => write!(f, "ELF64"),
            ElfClass::Unknown(x) => write!(f, "Unknown ELF class ({})", x),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

impl fmt::Display for Endianness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Endianness::LittleEndian => write!(f, "Little Endian"),
            Endianness::BigEndian => write!(f, "Big Endian"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Abi {
    SystemV,
    NetBSD,
    Linux,
    FreeBSD,
    OpenBSD,
    Arm,
    Standalone,
    Unknown(u8),
}

impl fmt::Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Abi::SystemV => write!(f, "UNIX System V"),
            Abi::NetBSD => write!(f, "NetBSD"),
            Abi::Linux => write!(f, "Linux"),
            Abi::FreeBSD => write!(f, "FreeBSD"),
            Abi::OpenBSD => write!(f, "OpenBSD"),
            Abi::Arm => write!(f, "ARM EABI"),
            Abi::Standalone => write!(f, "Standalone Embedded ABI"),
            Abi::Unknown(x) => write!(f, "Unknown ABI ({})", x),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Architecture {
    None,
    Sparc,
    Arm,
    X86,
    X86_64,
    RiscV,
    AArch64,
    Unknown(u16),
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::None => write!(f, "No specific architecture"),
            Architecture::Sparc => write!(f, "SPARC"),
            Architecture::Arm => write!(f, "ARM"),
            Architecture::X86 => write!(f, "Intel x86"),
            Architecture::X86_64 => write!(f, "AMD64 / x86-64"),
            Architecture::RiscV => write!(f, "RISC-V"),
            Architecture::AArch64 => write!(f, "ARM64 (AArch64)"),
            Architecture::Unknown(x) => write!(f, "Unknown architecture ({})", x),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum BinaryType {
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core,
    Unknown(u16),
}

impl fmt::Display for BinaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryType::None => write!(f, "No specific Binary Type"),
            BinaryType::Relocatable => write!(f, "Relocatable file"),
            BinaryType::Executable => write!(f, "Executable file"),
            BinaryType::SharedObject => write!(f, "Shared Object"),
            BinaryType::Core => write!(f, "Core Dump"),
            BinaryType::Unknown(x) => write!(f, "Unknown binary type ({})", x),
        }
    }
}

/// Basic filesystem metadata for the analyzed file.
#[derive(Debug, PartialEq, Serialize)]
pub struct FileMetadata {
    pub name: String,
    pub clean_path: String,
    pub size: u64,
}

/// Parsed subset of the ELF header used by Lyrem.
#[derive(Debug, PartialEq, Serialize)]
pub struct ElfHeaderMetadata {
    pub class: ElfClass,
    pub endianness: Endianness,
    pub abi: Abi,
    pub architecture: Architecture,
    pub binary_type: BinaryType,
    pub entry_point: u64,
}

/// Top-level metadata extracted from an ELF file.
///
/// This structure groups file-level information, the ELF header, Program
/// Headers, and Dynamic Section entries into a single representation used by
/// Lyrem's output and security analysis layers.
#[derive(Debug, PartialEq, Serialize)]
pub struct ElfMetadata {
    pub file: FileMetadata,
    pub header: ElfHeaderMetadata,
    pub prog_header: Vec<ProgramHeader>,
    pub dyn_entry: Vec<DynamicEntry>,
}
