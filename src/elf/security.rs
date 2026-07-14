use core::fmt;

use serde::Serialize;

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

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
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

/// Security-oriented summary of an analyzed ELF binary.
///
/// This structure is derived from the ELF header, Program Headers, and Dynamic
/// Section entries. It summarizes common binary hardening properties such as
/// NX, PIE, RELRO, and RWX memory mappings.
#[derive(Debug, PartialEq, Serialize)]
pub struct SecurityAnalysis {
    /// No-eXecute stack status.
    pub nx: Status,
    ///Position Independent Executable status.
    pub pie: Status,
    /// RELRO protection status.
    pub relro: RelroStatus,
    /// Program Headers whose segments are readable, writable, and executable.
    pub rwx_segments: Vec<ProgramHeader>,
    /// Whether the ELF contains a `PT_DYNAMIC` Program Header.
    pub has_dynamic_segment: bool,
    /// Whether the ELF contains a `PT_INTERP` Program Header.
    pub has_interpreter: bool,
}

/// Builds a security-oriented analysis from parsed ELF metadata.
///
/// This function does not read the ELF file directly. It inspects the parsed
/// ELF header, Program Headers, and Dynamic Section entries to infer hardening
/// properties such as NX, PIE, RELRO, dynamic linking, interpreter presence,
/// and RWX segments.
pub fn analyze(metadata: &ElfMetadata) -> SecurityAnalysis {
    let bin = &metadata.header.binary_type;
    let entry = &metadata.dyn_entry;
    let rwx_header: Vec<ProgramHeader> = Vec::new();
    let mut response = SecurityAnalysis {
        nx: Status::Unknown,
        pie: Status::Unknown,
        relro: RelroStatus::None,
        rwx_segments: rwx_header,
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
                match bin {
                    Executable => response.pie = Disabled,
                    SharedObject => response.pie = Enabled,
                    _ => response.pie = Unknown,
                }
            }
            ProgramType::GnuRelro => {
                response.relro = Partial;
                for i in entry {
                    if i.tag == BindNow {
                        response.relro = Full;
                        break;
                    }
                    // Full RELRO requires GNU_RELRO plus immediate symbol binding.
                    if i.tag == Flags && (i.value & 0x8) != 0 {
                        response.relro = Full;
                        break;
                    }
                    if i.tag == Flags1 && (i.value & 0x1) != 0 {
                        response.relro = Full;
                        break;
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
            response.rwx_segments.push(program_header.clone())
        }
    }
    response
}
