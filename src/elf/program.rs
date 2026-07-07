use std::fmt;

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

impl fmt::Display for ProgramType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramType::Null => write!(f, "NULL"),
            ProgramType::Load => write!(f, "LOAD"),
            ProgramType::Dynamic => write!(f, "DYNAMIC"),
            ProgramType::Interp => write!(f, "INTERP"),
            ProgramType::Note => write!(f, "NOTE"),
            ProgramType::Shlib => write!(f, "SHLIB"),
            ProgramType::Phdr => write!(f, "PHDR"),
            ProgramType::Tls => write!(f, "TLS"),
            ProgramType::GnuStack => write!(f, "GNU_STACK"),
            ProgramType::GnuRelro => write!(f, "GNU_RELRO"),
            ProgramType::GnuProperty => write!(f, "GNU_PROPERTY"),
            ProgramType::GnuEhFrame => write!(f, "GNU_EH_FRAME"),
            ProgramType::Unknown(value) => write!(f, "Unknown ({:#X})", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgramFlags {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl fmt::Display for ProgramFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            if self.readable { "R" } else { "-" },
            if self.writable { "W" } else { "-" },
            if self.executable { "E" } else { "-" },
        )
    }
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
