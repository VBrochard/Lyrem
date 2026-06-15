use std::path::PathBuf;

pub enum ElfClass {
    Elf32,
    Elf64,
    Unknown(u8)
}

pub enum Abi{
    SystemV,
    NetBSD,
    Linux,
    FreeBSD,
    OpenBSD,
    Standalone,
    Unknown(u8)
}

pub enum Architecture {
    None,
    Sparc,
    Arm,
    X86,
    X86_64,
    RiscV,
    AArch64,
    Unknown(u16)

}

pub enum BinaryType{
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core,
    Unknown(u16)
}

pub struct FileMetadata {
    pub name : String,
    pub path: PathBuf,
    pub size : u64
}

pub struct ElfHeaderMetadata{
    pub class : ElfClass,
    pub abi: Abi,
    pub architecture: Architecture,
    pub binary_type : BinaryType,
    pub entry_point : u64

}

pub struct ElfMetadata{
    pub file : FileMetadata,
    pub header : ElfHeaderMetadata
}