use core::fmt;

use crate::elf::{
    dynamic::DynamicTag::{BindNow, Flags, Flags1},
    metadata::{
        BinaryType::{Executable, SharedObject},
        ElfMetadata,
    },
    program::{ProgramHeader, ProgramType},
    security::{
        RelroStatus::{Full, Partial},
        Status::{Disabled, Enabled, Unknown},
    },
};

#[derive(Debug, PartialEq)]
pub enum Status {
    Enabled,
    Disabled,
    Unknown,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Enabled => write!(f, "Enabled"),
            Status::Disabled => write!(f, "Disabled"),
            Status::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RelroStatus {
    None,
    Partial,
    Full,
    Unknown,
}

impl fmt::Display for RelroStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelroStatus::None => write!(f, "None"),
            RelroStatus::Partial => write!(f, "Partial"),
            RelroStatus::Full => write!(f, "Full"),
            RelroStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DynamicLinkingStatus {
    Dynamic,
    Static,
    Unknown,
}

impl fmt::Display for DynamicLinkingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynamicLinkingStatus::Dynamic => write!(f, "Dynamic"),
            DynamicLinkingStatus::Static => write!(f, "Static"),
            DynamicLinkingStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SecurityAnalysis {
    pub nx: Status,
    pub pie: Status,
    pub relro: RelroStatus,
    pub rwx_segment: Vec<ProgramHeader>,
    pub has_dynamic_segment: bool,
    pub has_interpreter: bool,
}

pub fn analyze(metadata: &ElfMetadata) -> SecurityAnalysis {
    let bin = &metadata.header.binary_type;
    let entry = &metadata.dyn_entry;
    let rwx_header: Vec<ProgramHeader> = Vec::new();
    let mut response = SecurityAnalysis {
        nx: Status::Unknown,
        pie: Status::Unknown,
        relro: RelroStatus::None,
        rwx_segment: rwx_header,
        has_dynamic_segment: false,
        has_interpreter: false,
    };
    for program_header in &metadata.prog_header {
        match &program_header.program_type {
            ProgramType::GnuStack => {
                if program_header.flags.executable {
                    response.nx = Disabled
                } else {
                    response.nx = Enabled
                }
            }
            ProgramType::Interp => {
                response.has_interpreter = true;
                if bin == &Executable {
                    response.pie = Disabled
                }
                if bin == &SharedObject {
                    response.pie = Enabled
                } else {
                    response.pie = Unknown
                }
            }
            ProgramType::GnuRelro => {
                response.relro = Partial;
                for i in entry {
                    if i.tag == BindNow {
                        response.relro = Full
                    }
                    if i.tag == Flags && i.value == 0x8 {
                        response.relro = Full
                    }
                    if i.tag == Flags1 && i.value == 0x1 {
                        response.relro = Full
                    }
                }
            }
            ProgramType::Dynamic => response.has_dynamic_segment = true,
            _ => {}
        }
        if program_header.flags.executable
            && program_header.flags.writable
            && program_header.flags.readable
        {
            response.rwx_segment.push(program_header.clone())
        }
    }
    response
}
