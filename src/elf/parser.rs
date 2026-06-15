use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::elf::metadata::ElfMetadata;
use crate::elf::parser::ElfError::NotAnElfFile;

enum ElfError {
    Io(io::Error),
    NotAnElfFile,
}

impl From<io::Error> for ElfError {
    fn from(error: io::Error) -> ElfError {
        ElfError::Io(error)
    }
}

fn parse_elf(chemin: PathBuf) -> Result<(), ElfError> {
    let mut f = File::open(chemin)?;
    check_magic(&mut f)?;
    Ok(())
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
