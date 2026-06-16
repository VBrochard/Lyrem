use std::path::PathBuf;

#[derive(Debug)]
pub enum ElfClass {
    Elf32,
    Elf64,
    Unknown(u8),
}

#[derive(Debug)]
pub enum Endianess {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum BinaryType {
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core,
    Unknown(u16),
}

#[derive(Debug)]
pub struct FileMetadata {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug)]
pub struct ElfHeaderMetadata {
    pub class: ElfClass,
    pub endianess: Endianess,
    pub abi: Abi,
    pub architecture: Architecture,
    pub binary_type: BinaryType,
    pub entry_point: u64,
}

#[derive(Debug)]
pub struct ElfMetadata {
    pub file: FileMetadata,
    pub header: ElfHeaderMetadata,
}
