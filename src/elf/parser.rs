use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::elf::metadata::Abi;
use crate::elf::metadata::Architecture;
use crate::elf::metadata::BinaryType;

use crate::elf::metadata::ElfClass::{self};
use crate::elf::metadata::ElfHeaderMetadata;
use crate::elf::metadata::Endianess::{self};
use crate::elf::parser::ElfError::NotAnElfFile;

#[derive(Debug)]
pub enum ElfError {
    Io(io::Error),
    NotAnElfFile,
}

impl From<io::Error> for ElfError {
    fn from(error: io::Error) -> ElfError {
        ElfError::Io(error)
    }
}

pub fn parse_elf(chemin: &Path) -> Result<ElfHeaderMetadata, ElfError> {
    let mut f = File::open(chemin)?;
    check_magic(&mut f)?;
    let class_byte = read_u8(&mut f)?;
    let class = parse_class(class_byte);
    let endian_byte = read_u8(&mut f)?;
    let endianess = parse_endian(endian_byte);
    let header = ElfHeaderMetadata {
        class,
        endianess,
        abi: Abi::Unknown(0),
        architecture: Architecture::Unknown(0),
        binary_type: BinaryType::Unknown(0),
        entry_point: 0,
    };
    Ok(header)
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

fn parse_endian(endian: u8) -> Endianess {
    match endian {
        1 => Endianess::LittleEndian,
        2 => Endianess::BigEndian,
        x => Endianess::Unknown(x),
    }
}

fn read_u8(desc: &mut File) -> Result<u8, ElfError> {
    let mut buffer = [0];
    desc.read_exact(&mut buffer)?;
    Ok(buffer[0])
}
