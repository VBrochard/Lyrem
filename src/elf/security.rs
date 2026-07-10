use core::fmt;

use crate::elf::{
    metadata::{
        BinaryType::{Executable, SharedObject},
        ElfMetadata,
    },
    program::{ProgramHeader, ProgramType},
    security::Status::{Disabled, Enabled, Unknown},
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
    Present,
    Unknown,
}

impl fmt::Display for RelroStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelroStatus::None => write!(f, "None"),
            RelroStatus::Present => write!(f, "Present"),
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
    let mut response = SecurityAnalysis {
        nx: Status::Unknown,
        pie: Status::Unknown,
        relro: RelroStatus::Unknown,
        rwx_segment: Vec::new(),
        has_dynamic_segment: false,
        has_interpreter: false,
    };
    response.pie = Unknown;
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
                if bin == &SharedObject {
                    response.pie = Enabled
                } else {
                    response.pie = Unknown
                }
                if bin == &Executable {
                    response.pie = Disabled
                }
            }
            _ => {}
        }
    }
    response
}
