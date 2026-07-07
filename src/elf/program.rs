#[derive(Debug, PartialEq)]
pub enum ProgramType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    GnuStack,
    GnuRelro,
    GnuProperty,
    GnuEhFrame,
    Unknown(u32),
}

#[derive(Debug, PartialEq)]
pub struct ProgramFlags {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

#[derive(Debug, PartialEq)]
pub struct ProgramHeader {
    pub program_type: ProgramType,
    pub flags: ProgramFlags,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub align: u64,
}
